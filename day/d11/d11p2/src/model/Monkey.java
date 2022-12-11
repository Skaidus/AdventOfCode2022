package model;

import java.util.LinkedList;

public class Monkey
{
    LinkedList<Long> items;
    Operation op;
    long divisor, ZZn;
    Monkey TrueMonkey, FalseMonkey;
    long inspections = 0;

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

    public void setRing(int ZZn){
        this.ZZn = ZZn;
    }

    public void throw_item(){
        while(!this.items.isEmpty()){
            final long old_v = items.remove();
            final long new_v = op.apply(old_v) % ZZn;
            this.getReceiver(new_v).receive_item(new_v);
            inspections++;
        }
    }

    public long getInspections(){
        return this.inspections;
    }

    public void receive_item(long item){
        this.items.add(item);
    }

    private Monkey getReceiver(final long item){
        return (item % divisor == 0)? TrueMonkey : FalseMonkey;
    }
}
