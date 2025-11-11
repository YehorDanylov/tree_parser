use anyhow::{Context, Result};
use std::env;
use std::fs;
use tree_parser::{evaluate, parse_expression};

fn print_help() {
    println!(
        r#"Tree Parser CLI

Usage (via cargo):
  cargo run -- parse <file>   - Зчитати вираз із файлу та вивести AST
  cargo run -- eval <file>    - Зчитати вираз із файлу та обчислити результат
  cargo run -- help           - Показати довідку
  cargo run -- about          - Інформація про автора і проєкт

Usage (via Makefile):
  make parse <file>           - Зчитати вираз із файлу та вивести AST
  make eval <file>            - Зчитати вираз із файлу та обчислити результат
  make help                   - Показати довідку
  make about                  - Інформація про автора і проєкт
"#
    );
}

fn print_about() {
    println!("Tree Parser — Expression parser in Rust");
    println!("Created by Yehor Danylov, 2025");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            let filename = args.get(2).context("Error: Missing filename")?;
            let content = fs::read_to_string(filename)
                .with_context(|| format!("Cannot read file '{}'", filename))?;
            let expr = parse_expression(&content)
                .with_context(|| format!("Invalid expression in file '{}'", filename))?;
            expr.print_tree();
        }

        "eval" => {
            let filename = args.get(2).context("Error: Missing filename")?;
            let content = fs::read_to_string(filename)
                .with_context(|| format!("Cannot read file '{}'", filename))?;
            let expr = parse_expression(&content)
                .with_context(|| format!("Invalid expression in file '{}'", filename))?;
            let result = evaluate(&expr);
            println!("Result: {}", result);
        }

        "help" => print_help(),

        "about" => print_about(),

        other => {
            eprintln!("Unknown command: {}", other);
            print_help();
        }
    }

    Ok(())
}
