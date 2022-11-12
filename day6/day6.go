package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
)

type LineIterator struct {
	reader *bufio.Reader
}

func NewLineIterator(rd io.Reader) *LineIterator {
	return &LineIterator{
		reader: bufio.NewReader(rd),
	}
}

func (ln *LineIterator) Next() ([]byte, error) {
	var bytes []byte
	for {
		line, isPrefix, err := ln.reader.ReadLine()
		if err != nil {
			return nil, err
		}
		bytes = append(bytes, line...)
		if !isPrefix {
			break
		}
	}
	return bytes, nil
}

func main() {
	ln := NewLineIterator(os.Stdin)

	res := 0
	s := make(map[rune]bool)
	for {
		line, err := ln.Next()
		if err != nil {
			if err == io.EOF {
				break
			} else {
				log.Fatal(err)
			}
		}
		var exists = true
		line_st := fmt.Sprintf("%s", line)
		fmt.Printf("Processing %s\n", line_st)
		if len(line_st) > 0 {
			for _, c := range line_st {
				s[c] = exists
			}
		} else {
			fmt.Println(s)
			res = res + len(s)
			s = make(map[rune]bool)
		}

	}
	fmt.Println(res)
}
