package model;

public class Operation {
    int v1, v2;
    boolean is_sum;
    public int apply(int v){
        int v1p = (v1==0)? v: v1;
        int v2p = (v2==0)? v: v2;
        return (is_sum)? v1p+v2p: v1p*v2p;
    }

    public Operation(int v1, int v2, char op){
        this.v1 = v1;
        this.v2 = v2;
        this.is_sum = (op == '+');
    }
}
