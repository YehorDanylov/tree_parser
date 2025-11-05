use tree_parser::{parse_expression, evaluate, Expr};
fn main() {
let expr = parse_expression("3 + 5 * (2 - 8) / 4").unwrap();
expr.print_tree();

}
