package model;

public interface ICrateMover {
    public void operate(int blocks, int from, int to);
    public void addCrate(Character crate, int stack);
    public String getTop();
}
