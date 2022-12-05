import java.util.ArrayList;
import java.util.Stack;

public class CraneOperator implements ICraneOperator {
    private ArrayList<Stack<Character>> stacks;
    public CraneOperator(int n_stacks){
        this.stacks = new ArrayList<>(n_stacks);
        for(int i = 0; i< n_stacks; i++) this.stacks.add(new Stack<>());
    }
    @Override
    public void operate(int blocks, int from, int to) {
        Stack<Character> from_stack = stacks.get(from);
        Stack<Character> to_stack = stacks.get(to);
        for(int i = 0; i < blocks; i++) to_stack.push(from_stack.pop());
    }

    @Override
    public void addCrate(Character crate, int stack) {
        this.stacks.get(stack).push(crate);
    }

    @Override
    public String getTop() {
        StringBuilder top = new StringBuilder();
        for (Stack<Character> stack : stacks) if (stack.isEmpty()) top.append(stack.pop());
        return top.toString();
    }
}
