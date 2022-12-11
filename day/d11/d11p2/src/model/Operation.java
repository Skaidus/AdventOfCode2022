package model;

public class Operation {
    long v1, v2;
    boolean is_sum;
    public long apply(long v){
        long v1p = (v1==0)? v: v1;
        long v2p = (v2==0)? v: v2;
        return (is_sum)? v1p+v2p: v1p*v2p;
    }

    public Operation(long v1, long v2, char op){
        this.v1 = v1;
        this.v2 = v2;
        this.is_sum = (op == '+');
    }
}
