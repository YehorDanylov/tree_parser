#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    BinaryOp {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {

    pub fn print_tree(&self) {
        println!("\nExpression: {}\n", self.to_infix());
        Self::print_node(self, "".to_string(), true);
        println!();
    }

    /// Рекурсивний друк вузлів
    fn print_node(expr: &Expr, prefix: String, is_last: bool) {
        let connector = if is_last { "└── " } else { "├── " };
        print!("{}", prefix);
        print!("{}", connector);

        match expr {
            Expr::Number(n) => println!("{}", n),
            Expr::BinaryOp { op, left, right } => {
                println!("{}", op);

                let new_prefix = prefix + if is_last { "    " } else { "│   " };

                let children = vec![left.as_ref(), right.as_ref()];

                for (i, child) in children.iter().enumerate() {
                    let last = i == children.len() - 1;
                    Self::print_node(child, new_prefix.clone(), last);
                }
            }
        }
    }

    /// Вивід виразу у звичайному інфіксному вигляді: (a + b)
    pub fn to_infix(&self) -> String {
        match self {
            Expr::Number(n) => format!("{}", n),
            Expr::BinaryOp { op, left, right } => {
                format!("({} {} {})", left.to_infix(), op, right.to_infix())
            }
        }
    }
}


// Додаємо Display щоб виводити числа та оператори красиво
use std::fmt;
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::BinaryOp { op, .. } => write!(f, "({})", op),
        }
    }
}


pub fn parse_expression(input: &str) -> Result<Expr, String> {
    let mut tokens = tokenize(input)?;
    parse_expr(&mut tokens)
}

fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    for ch in input.chars() {
        if ch.is_whitespace() {
            continue;
        } else if ch.is_ascii_digit() {
            number.push(ch);
        } else {
            if !number.is_empty() {
                tokens.push(number.clone());
                number.clear();
            }
            tokens.push(ch.to_string());
        }
    }
    if !number.is_empty() {
        tokens.push(number);
    }

    Ok(tokens)
}

fn parse_expr(tokens: &mut Vec<String>) -> Result<Expr, String> {
    parse_binary_op(tokens, parse_term, &['+', '-'])
}

fn parse_term(tokens: &mut Vec<String>) -> Result<Expr, String> {
    parse_binary_op(tokens, parse_factor, &['*', '/'])
}

fn parse_binary_op<F>(tokens: &mut Vec<String>, subparser: F, ops: &[char]) -> Result<Expr, String>
where
    F: Fn(&mut Vec<String>) -> Result<Expr, String>,
{
    let mut left = subparser(tokens)?;
    while let Some(op) = tokens.first().and_then(|s| s.chars().next()) {
        if ops.contains(&op) {
            tokens.remove(0);
            let right = subparser(tokens)?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_factor(tokens: &mut Vec<String>) -> Result<Expr, String> {
    if tokens.is_empty() {
        return Err("Unexpected end".into());
    }

    let token = tokens.remove(0);

    if token == "(" {
        let expr = parse_expr(tokens)?;
        if tokens.remove(0) != ")" {
            return Err("Missing ')'".into());
        }
        Ok(expr)
    } else if let Ok(num) = token.parse::<f64>() {
        Ok(Expr::Number(num))
    } else {
        Err(format!("Unexpected token: {}", token))
    }
}

pub fn evaluate(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinaryOp { op, left, right } => {
            let l = evaluate(left);
            let r = evaluate(right);
            match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                _ => unreachable!(),
            }
        }
    }
}
