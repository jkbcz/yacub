package scanner

import (
	"fmt"
	"strconv"
	"unicode"
	"unicode/utf8"

	"github.com/jkbcz/yacub/token"
)

type Scanner struct {
	source []byte

	current   int
	start     int
	line      int
	lineStart int
}

func NewScanner(source []byte) *Scanner {

	return &Scanner{
		source:    source,
		current:   0,
		start:     0,
		line:      1,
		lineStart: 0,
	}
}

func (s *Scanner) ScanAll() ([]token.Token, error) {
	var tokens []token.Token
	for {
		t, err := s.Scan()
		if err != nil {
			return tokens, err
		}
		tokens = append(tokens, t)
		if t.Type == token.EOF {
			return tokens, nil
		}
	}
}

func (s *Scanner) Scan() (token.Token, error) {
	s.start = s.current

	if s.isAtEnd() {
		return s.token(token.EOF, nil), nil
	}
	ch := s.advance()
	switch ch {
	case '(':
		return s.token(token.LEFT_PAREN, nil), nil
	case ')':
		return s.token(token.RIGHT_PAREN, nil), nil
	case '-':
		return s.token(token.MINUS, nil), nil
	case '+':
		return s.token(token.PLUS, nil), nil
	case '*':
		return s.token(token.STAR, nil), nil
	case '/':
		return s.token(token.SLASH, nil), nil
	case '!':
		return s.token(s.match2('=', token.BANG_EQUAL, token.BANG), nil), nil
	case '=':
		return s.token(s.match2('=', token.EQUAL_EQUAL, token.EQUAL), nil), nil
	case '<':
		return s.token(s.match2('=', token.LESS_EQUAL, token.LESS), nil), nil
	case '>':
		return s.token(s.match2('=', token.GREATER_EQUAL, token.GREATER), nil), nil
	case ' ':
	case '\r':
	case '\t':
		break
	case '\n':
		s.line++
		s.lineStart = s.current
	default:
		if isDigit(ch) {
			return s.scanNumber()
		}
		if isAlpha(ch) {
			return s.scanIdentifier()
		}
		return token.Token{}, s.errorf("invalid character: '%c'", ch)
	}

	return s.Scan()
}

func (s *Scanner) scanNumber() (token.Token, error) {
	for isDigit(s.peek()) {
		s.advance()
	}

	value, err := strconv.Atoi(string(s.source[s.start:s.current]))
	return s.token(token.NUMBER, value), err
}

func (s *Scanner) scanIdentifier() (token.Token, error) {
	for isAlphaNumeric(s.peek()) {
		s.advance()
	}
	return s.token(token.IDENTIFIER, nil), nil
}

func (s *Scanner) advance() rune {
	r, size := utf8.DecodeRune(s.source[s.current:])
	s.current += size
	return r
}

func (s *Scanner) peek() rune {
	r, _ := utf8.DecodeRune(s.source[s.current:])
	return r
}

func (s *Scanner) match2(expected rune, t1, t2 token.Type) token.Type {
	if s.isAtEnd() {
		return t2
	}
	if s.peek() != expected {
		return t2
	}
	s.advance()
	return t1
}

func (s *Scanner) isAtEnd() bool {
	return s.current >= len(s.source)
}

func (s *Scanner) token(typ token.Type, lit any) token.Token {
	lexeme := string(s.source[s.start:s.current])
	return token.Token{
		Type:    typ,
		Lexeme:  lexeme,
		Literal: lit,
		Line:    s.line,
		Column:  s.column(),
	}
}

func (s *Scanner) error(msg string) Error {
	return Error{
		Line:   s.line,
		Column: s.column(),
		Msg:    msg,
	}
}

func (s *Scanner) column() int {
	return s.start - s.lineStart + 1
}

func (s *Scanner) errorf(msgFormat string, args ...any) Error {
	return s.error(fmt.Sprintf(msgFormat, args...))
}

func isDigit(a rune) bool {
	return a >= '0' && a <= '9'
}

func isAlpha(a rune) bool {
	return unicode.IsLetter(a)
}

func isAlphaNumeric(a rune) bool {
	return isAlpha(a) || isDigit(a)
}
