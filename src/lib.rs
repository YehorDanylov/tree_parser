use std::fmt;
use thiserror::Error;

/// Абстрактне синтаксичне дерево (AST) для арифметичних виразів.
///
/// # Вузли AST
/// - `Number(f64)` — число
/// - `BinaryOp { op, left, right }` — бінарна операція (`+`, `-`, `*`, `/`)
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Числовий вузол
    Number(f64),

    /// Бінарна операція
    BinaryOp {
        /// Оператор: '+', '-', '*', '/'
        op: char,
        /// Ліве піддерево
        left: Box<Expr>,
        /// Праве піддерево
        right: Box<Expr>,
    },
}

/// Можливі помилки парсингу
#[derive(Error, Debug)]
pub enum ParseError {
    /// Неочікуваний кінець вводу
    #[error("Unexpected end of input")]
    UnexpectedEnd,

    /// Неочікуваний токен
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    /// Відсутня закриваюча дужка
    #[error("Missing closing parenthesis")]
    MissingClosingParenthesis,
}

impl Expr {
    /// Виводить дерево AST у консоль
    ///
    /// # Приклад
    /// ```
    /// let expr = tree_parser::parse_expression("2 + 3 * 4").unwrap();
    /// expr.print_tree();
    /// ```
    pub fn print_tree(&self) {
        println!("\nExpression: {}\n", self.to_infix());
        Self::print_node(self, "".to_string(), true);
        println!();
    }

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

    /// Повертає рядкове представлення виразу у звичайному інфіксному вигляді
    ///
    /// # Приклад
    /// ```
    /// let expr = tree_parser::parse_expression("2 + 3").unwrap();
    /// assert_eq!(expr.to_infix(), "(2 + 3)");
    /// ```
    pub fn to_infix(&self) -> String {
        match self {
            Expr::Number(n) => format!("{}", n),
            Expr::BinaryOp { op, left, right } => {
                format!("({} {} {})", left.to_infix(), op, right.to_infix())
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::BinaryOp { op, .. } => write!(f, "({})", op),
        }
    }
}

/// Парсить арифметичний вираз у рядку та повертає AST
///
/// # Граматика
///
/// Expr   = Term { ("+" | "-") Term } ;
/// Term   = Factor { ("*" | "/") Factor } ;
/// Factor = Number | "(" Expr ")" ;
/// Number = digit { digit } ;
///
/// # Приклад
/// ```
/// let expr = tree_parser::parse_expression("3 + 5 * (2 - 8) / 4").unwrap();
/// ```
pub fn parse_expression(input: &str) -> Result<Expr, ParseError> {
    let mut tokens = tokenize(input)?;
    parse_expr(&mut tokens)
}

/// Токенізація рядка у вектор токенів
fn tokenize(input: &str) -> Result<Vec<String>, ParseError> {
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

/// Реалізація правила граматики Expr = Term { ("+" | "-") Term }
fn parse_expr(tokens: &mut Vec<String>) -> Result<Expr, ParseError> {
    parse_binary_op(tokens, parse_term, &['+', '-'])
}

/// Реалізація правила граматики Term = Factor { ("*" | "/") Factor }
fn parse_term(tokens: &mut Vec<String>) -> Result<Expr, ParseError> {
    parse_binary_op(tokens, parse_factor, &['*', '/'])
}

/// Парсинг бінарної операції
fn parse_binary_op<F>(
    tokens: &mut Vec<String>,
    subparser: F,
    ops: &[char],
) -> Result<Expr, ParseError>
where
    F: Fn(&mut Vec<String>) -> Result<Expr, ParseError>,
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

/// Реалізація правила граматики Factor = Number | "(" Expr ")"
fn parse_factor(tokens: &mut Vec<String>) -> Result<Expr, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::UnexpectedEnd);
    }

    let token = tokens.remove(0);

    if token == "(" {
        let expr = parse_expr(tokens)?;
        if tokens.is_empty() || tokens.remove(0) != ")" {
            return Err(ParseError::MissingClosingParenthesis);
        }
        Ok(expr)
    } else if let Ok(num) = token.parse::<f64>() {
        Ok(Expr::Number(num))
    } else {
        Err(ParseError::UnexpectedToken(token))
    }
}

/// Обчислює значення AST
///
/// # Приклад
/// ```
/// let expr = tree_parser::parse_expression("3 + 5").unwrap();
/// assert_eq!(tree_parser::evaluate(&expr), 8.0);
/// ```
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
