package main

import (
	"fmt"
	"os"
)

func findStart(size int) {
	line, err := os.ReadFile("input.txt")
	if err != nil {
		panic("Could not open file")
	}
	h := NewHeaderStack(line, size)
	body := line[size:]
	for _, v := range body {
		h.Push(v)
		if h.AllDifferent() {
			fmt.Printf("Start after %v\n", size+h.Pushes)
			break
		}
	}
}
