package scanner

import "fmt"

type Error struct {
	Line   int
	Column int
	Msg    string
}

func (e Error) Error() string {
	return fmt.Sprintf("[%d:%d] Error: %s", e.Line, e.Column, e.Msg)
}
