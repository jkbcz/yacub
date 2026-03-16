# YACUB

Yet Another Compiler with Undefined Behavior

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

## Running

```bash
go run cmd/yacub/main.go
```

Type in an expression into the STDIN and the program will display scanned tokens

Example expression `a + 5 - 10 + 家`
