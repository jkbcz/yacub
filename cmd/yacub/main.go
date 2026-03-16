package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"

	"github.com/jkbcz/yacub/scanner"
)

func main() {
	r := bufio.NewReader(os.Stdin)
	for {
		data, _, err := r.ReadLine()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatal(err)
		}
		s := scanner.NewScanner(data)
		tokens, err := s.ScanAll()
		if err != nil {
			log.Println(err)
			continue
		}
		for _, token := range tokens {
			fmt.Println(token)
		}
	}
}
