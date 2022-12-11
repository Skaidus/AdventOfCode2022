package model;

import java.util.Stack;

public class CrateMover9001 extends CrateMover9000 {
    private Stack<Character> cache;
    public CrateMover9001(int n_stacks){
        super(n_stacks);
        cache = new Stack<>();
    }
    @Override
    public void operate(int blocks, int from, int to) {
        Stack<Character> from_stack = stacks.get(from);
        Stack<Character> to_stack = stacks.get(to);
        for(int i = 0; i < blocks; i++) cache.push(from_stack.pop());
        while(!cache.isEmpty()) to_stack.push(cache.pop());
    }
}
