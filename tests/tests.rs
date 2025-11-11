use anyhow::Result;
use tree_parser::{Expr, evaluate, parse_expression, ParseError};

#[test]
fn test_simple_addition_ast() -> Result<()> {
    let expr = parse_expression("2 + 3")?;
    expr.print_tree();
    assert_eq!(
        expr,
        Expr::BinaryOp {
            op: '+',
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        }
    );
    assert_eq!(expr.to_infix(), "(2 + 3)");
    Ok(())
}

#[test]
fn test_operator_precedence_ast() -> Result<()> {
    let expr = parse_expression("2 + 3 * 4")?;
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
    assert_eq!(evaluate(&expr), 14.0);
    Ok(())
}

#[test]
fn test_with_parentheses_ast() -> Result<()> {
    let expr = parse_expression("(2 + 3) * 4")?;
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
    assert_eq!(evaluate(&expr), 20.0);
    Ok(())
}

#[test]
fn test_complex_expression_ast() -> Result<()> {
    let expr = parse_expression("3 + 5 * (2 - 8) / 4")?;
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
            }),
        }
    );
    let result = evaluate(&expr);
assert!((result - (-4.5)).abs() < 1e-6);
    Ok(())
}

#[test]
fn test_evaluate_numeric() -> Result<()> {
    let expr = parse_expression("3 + 5 * (2 - 8) / 4")?;
    let result = evaluate(&expr);
    assert!((result + 4.5).abs() < 1e-6); 
    Ok(())
}


#[test]
fn test_unexpected_end_error() {
    let err = parse_expression("").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedEnd));
}

#[test]
fn test_missing_closing_parenthesis_error() {
    let err = parse_expression("(").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedEnd | ParseError::MissingClosingParenthesis));

    let err2 = parse_expression("2 + (3 * 4").unwrap_err();
    assert!(matches!(err2, ParseError::MissingClosingParenthesis));
}


#[test]
fn test_unexpected_token_error() {
    let err = parse_expression("2 + x").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedToken(tok) if tok == "x"));
}

#[test]
fn test_to_infix() -> Result<()> {
    let expr = parse_expression("1 + 2 * 3")?;
    assert_eq!(expr.to_infix(), "(1 + (2 * 3))");

    let expr2 = parse_expression("(1 + 2) * 3")?;
    assert_eq!(expr2.to_infix(), "((1 + 2) * 3)");
    Ok(())
}

#[test]
fn test_nested_parentheses() -> Result<()> {
    let expr = parse_expression("((1 + 2) * (3 + 4)) / 7")?;
    let result = evaluate(&expr);
    assert!((result - 3.0).abs() < 1e-6); 
    Ok(())
}

#[test]
fn test_multiple_operators_error() {
    let err = parse_expression("2 + + 3").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedToken(tok) if tok == "+"));
}

#[test]
fn test_invalid_token_error() {
    let err = parse_expression("2 + @").unwrap_err();
    assert!(matches!(err, ParseError::UnexpectedToken(tok) if tok == "@"));
}

#[test]
fn test_division_by_zero() -> Result<()> {
    let expr = parse_expression("10 / (5 - 5)")?;
    let result = evaluate(&expr);
    assert!(result.is_infinite() || result.is_nan());
    Ok(())
}

#[test]
fn test_long_complex_expression() -> Result<()> {
    let expr = parse_expression("1 + 2 - 3 * 4 / 2 + (5 - 6 + (7 * 8))")?;
    let result = evaluate(&expr);
    assert!((result - 52.0).abs() < 1e-6); 
    Ok(())
}


#[test]
fn test_only_number() -> Result<()> {
    let expr = parse_expression("42")?;
    assert_eq!(evaluate(&expr), 42.0);
    Ok(())
}
