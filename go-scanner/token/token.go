//go:generate go tool stringer -type=Type
package token

import "fmt"

type Type uint

const (
	// Single-character tokens.
	LEFT_PAREN Type = iota
	RIGHT_PAREN
	MINUS
	PLUS
	SLASH
	STAR

	// One or two character tokens.
	BANG
	BANG_EQUAL
	EQUAL
	EQUAL_EQUAL
	GREATER
	GREATER_EQUAL
	LESS
	LESS_EQUAL

	// Literals.
	IDENTIFIER
	NUMBER

	EOF
)

type Token struct {
	Type    Type
	Lexeme  string
	Literal any

	Line   int
	Column int
}

func (t Token) String() string {
	return fmt.Sprintf("[%d:%d] %s %s %v", t.Line, t.Column, t.Type, t.Lexeme, t.Literal)
}
