package main

type HeaderStack struct {
	buffer []byte
	Pushes int
}

func (h *HeaderStack) Push(top byte) {
	buffer := h.buffer
	for i := 1; i < len(buffer); i++ {
		buffer[i-1] = buffer[i]
	}
	buffer[len(buffer)-1] = top
	h.Pushes++
	return
}

func (h *HeaderStack) AllDifferent() bool {
	buffer := h.buffer
	for i, v := range buffer {
		for j := i + 1; j < len(buffer); j++ {
			if v == buffer[j] {
				return false
			}
		}
	}
	return true
}

func NewHeaderStack(start []byte, size int) HeaderStack {
	buffer := make([]byte, size)
	copy(buffer, start)
	return HeaderStack{
		buffer: buffer,
		Pushes: 0,
	}
}
