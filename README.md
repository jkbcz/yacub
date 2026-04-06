# YACUB

Yet Another Compiler with Undefined Behavior

## This repository contains two scanners (one with syntax highlighting):

### - Original YACUB - scanner written in Go

A lexical scanner that reads expressions from STDIN and outputs tokenized results. Supports Unicode identifiers and basic arithmetic/comparison operators.

How to run - `go run go-scanner/cmd/yacub/main.go`

Type in an expression and the program will display scanned tokens.

Example: `a + 5 - 10 + 家`

[README for go-scanner (YACUB)](./go-scanner/README.md)


### - Syntax Highlighter - scanner with highlighting capabilities, written in Rust

A Rust implementation of the YACUB scanner with syntax highlighting support. Processes source files and highlights tokens with color-coded output.

How to run - `cargo run <input_filname> <output_filename>`

## Tokens

| Name          | Definition                                                    |
| ------------- | ------------------------------------------------------------- |
| LEFT_PAREN    | (                                                             |
| RIGHT_PAREN   | )                                                             |
| MINUS         | -                                                             |
| PLUS          | +                                                             |
| SLASH         | \                                                             |
| STAR          | \*                                                            |
| BANG          | !                                                             |
| BANG_EQUAL    | !=                                                            |
| EQUAL         | =                                                             |
| EQUAL_EQUAL   | ==                                                            |
| GREATER       | >                                                             |
| GREATER_EQUAL | >=                                                            |
| LESS          | <                                                             |
| LESS_EQUAL    | <=                                                            |
| IDENTIFIER    | Sequence of letters (Unicode) or digits, starts with a letter |
| NUMBER        | Sequence of digits (0-9)                                      |
| EOF           | End of file                                                   |