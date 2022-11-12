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

func contains(s string, r rune) bool {
	for _, c := range s {
		if c == r {
			return true
		}
	}
	return false
}

func main() {
	ln := NewLineIterator(os.Stdin)

	res := 0
	s := make(map[rune]bool)

	is_first := true
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
			if is_first {
				is_first = false
				for _, c := range line_st {
					s[c] = exists
				}
			} else {
				fmt.Printf("Len(s)=%d\n", len(s))
				for k, _ := range s {
					fmt.Printf("Deleting %s??\n", k)
					if !contains(line_st, k) {
						fmt.Printf("Deleting %s\n", k)
						delete(s, k)
					}
				}
			}

		} else {
			fmt.Println(s)
			res = res + len(s)
			fmt.Printf("Adding %d \n", len(s))
			s = make(map[rune]bool)
			is_first = true
		}

	}
	fmt.Println(res)
}
