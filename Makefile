# Makefile for Tree Parser project

# Файли та налаштування
FILE ?= example.txt

# Запуск парсера і вивід AST
parse:
	cargo run -- parse $(FILE)

# Запуск парсера і обчислення результату
eval:
	cargo run -- eval $(FILE)

# Показати довідку
help:
	cargo run -- help

# Показати інформацію про автора
about:
	cargo run -- about

# Запуск всіх тестів
test:
	cargo test

# Форматування коду
fmt:
	cargo fmt

# Перевірка коду
clippy:
	cargo clippy -- -D warnings

