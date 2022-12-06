package main

import (
	"fmt"
	"os"
)

func main() {
	line, err := os.ReadFile("input.txt")
	if err != nil {
		panic("Could not open file")
	}
	h := NewHeaderStack(line, 4)
	body := line[4:]
	for _, v := range body {
		h.Push(v)
		if h.AllDifferent() {
			fmt.Printf("Start-of-packet after %v\n", 4+h.Pushes)
			break
		}
	}
}
