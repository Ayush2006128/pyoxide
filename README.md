# Pyoxide

A lightweight expression language interpreter written in Rust.

## Features

- Arithmetic expressions with `+`, `-`, `*`, `/`
- Variable assignment
- Parentheses for grouping
- Interactive REPL
- File execution mode

## Installation

```bash
cargo build --release
```

## Usage

### REPL Mode

Run without arguments to enter the interactive interpreter:

```bash
cargo run
```

Example session:
```
> x = 10
> y = 20
> x + y
30
> (x + y) * 2
60
```

### File Mode

Run a `.pyoxide` file:

```bash
cargo run -- example.pyoxide
```

Example `example.pyoxide`:
```python
radius = 5
area = 3.14 * radius * radius
area
```

## Language Syntax

```python
# Variable assignment
x = 10

# Expressions
y = x + 5        # addition
z = y - 3        # subtraction
result = z * 2   # multiplication
frac = result / 4  # division

# Parentheses
value = (x + y) * (z - 1)

# Print by evaluating
x + y
```

## Architecture

```
Lexer  -->  Parser  -->  Interpreter
```

- **Lexer**: Tokenizes input source code
- **Parser**: Builds AST from tokens
- **Interpreter**: Evaluates expressions

## License

MIT