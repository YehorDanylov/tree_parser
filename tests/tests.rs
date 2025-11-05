use tree_parser::{parse_expression, evaluate, Expr};

#[test]
fn test_simple_addition_ast() {
    let expr = parse_expression("2 + 3").unwrap();
    expr.print_tree();
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: '+',
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        }
    );
}

#[test]
fn test_operator_precedence_ast() {
    let expr = parse_expression("2 + 3 * 4").unwrap();
    expr.print_tree();
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: '+',
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::BinaryOp {
                op: '*',
                left: Box::new(Expr::Number(3.0)),
                right: Box::new(Expr::Number(4.0)),
            }),
        }
    );
}

#[test]
fn test_with_parentheses_ast() {
    let expr = parse_expression("(2 + 3) * 4").unwrap();
    expr.print_tree();
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: '*',
            left: Box::new(Expr::BinaryOp {
                op: '+',
                left: Box::new(Expr::Number(2.0)),
                right: Box::new(Expr::Number(3.0)),
            }),
            right: Box::new(Expr::Number(4.0)),
        }
    );
}

#[test]
fn test_complex_expression_ast() {
    let expr = parse_expression("3 + 5 * (2 - 8) / 4").unwrap();
expr.print_tree();
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: '+',
            left: Box::new(Expr::Number(3.0)),
            right: Box::new(Expr::BinaryOp {
                op: '/',
                left: Box::new(Expr::BinaryOp {
                    op: '*',
                    left: Box::new(Expr::Number(5.0)),
                    right: Box::new(Expr::BinaryOp {
                        op: '-',
                        left: Box::new(Expr::Number(2.0)),
                        right: Box::new(Expr::Number(8.0)),
                    }),
                }),
                right: Box::new(Expr::Number(4.0)),
            })
        }
    );
}

#[test]
fn test_evaluate_numeric() {
    let expr = parse_expression("3 + 5 * (2 - 8) / 4").unwrap();
    assert!((evaluate(&expr) + 4.5).abs() < 1e-6);
}