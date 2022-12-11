package model;

import java.util.LinkedList;

public class Monkey
{
    LinkedList<Integer> items;
    Operation op;
    int divisor;
    Monkey TrueMonkey, FalseMonkey;
    int inspections = 0;

    public Monkey(){
        items = new LinkedList<>();
    }

    public void setOperation(int v1, int v2, char op){
        this.op = new Operation(v1, v2, op);
    }

    public void setMonkeys(Monkey TrueMonkey, Monkey FalseMonkey){
        this.TrueMonkey = TrueMonkey;
        this.FalseMonkey = FalseMonkey;
    }

    public void setDivisor(int divisor){
        this.divisor = divisor;
    }

    public void throw_item(){
        while(!this.items.isEmpty()){
            final int old_v = items.remove();
            final int new_v = op.apply(old_v)/3;
            this.getReceiver(new_v).receive_item(new_v);
            inspections++;
        }
    }

    public int getInspections(){
        return this.inspections;
    }

    public void receive_item(int item){
        this.items.add(item);
    }

    private Monkey getReceiver(final int item){
        return (item % divisor == 0)? TrueMonkey : FalseMonkey;
    }
}
