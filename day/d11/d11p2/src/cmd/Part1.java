package cmd;
import model.Monkey;
import util.ReceiverConfig;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;
import java.util.Stack;

public class Part1 {
    public static void main(String[] args) {
        try {
            File myObj = new File("input.txt");
            Scanner myReader = new Scanner(myObj);
            int line_count = 0, true_monkey = 0, false_monkey;
            Stack<ReceiverConfig> rcs = new Stack<>();
            String[] list;
            ArrayList<Monkey> jungle = new ArrayList<>();
            int ZZn = 1;
            while (myReader.hasNextLine()) {
                String data = myReader.nextLine();
                switch (line_count) {
                    case 0 -> jungle.add(new Monkey());
                    case 1 -> {
                        list = data.substring(18).split(", ");
                        for (String s : list) jungle.get(jungle.size() - 1).receive_item(Integer.parseInt(s));
                    }
                    case 2 -> {
                        list = data.substring(19).split(" ");
                        int v1 = (list[0].equals("old")) ? 0 : Integer.parseInt(list[0]);
                        char op = list[1].charAt(0);
                        int v2 = (list[2].equals("old")) ? 0 : Integer.parseInt(list[2]);
                        jungle.get(jungle.size() - 1).setOperation(v1, v2, op);
                    }
                    case 3 -> {
                        int div = Integer.parseInt(data.substring(21));
                        jungle.get(jungle.size() - 1).setDivisor(div);
                        ZZn*=div;
                    }
                    case 4 -> true_monkey = Integer.parseInt(data.substring(29));
                    case 5 -> {
                        false_monkey = Integer.parseInt(data.substring(30));
                        rcs.push(new ReceiverConfig(false_monkey, true_monkey));
                    }
                    default -> line_count = -1;
                }
                line_count++;
            }
            ReceiverConfig cf;
            for(int i = jungle.size()-1; i >= 0; i--){
                cf = rcs.pop();
                jungle.get(i).setMonkeys(jungle.get(cf.true_monkey), jungle.get(cf.false_monkey));
                jungle.get(i).setRing(ZZn);
            }
            for(int i = 0; i < 10000; i++){
                for (Monkey monkey : jungle) {
                    monkey.throw_item();
                }
            }
            long max_1st = 0, max_2nd = 0, temp;
            for(Monkey m : jungle){
                temp = m.getInspections();
                if(temp > max_1st){
                    max_2nd = max_1st; max_1st = temp;
                } else if(temp > max_2nd){
                    max_2nd = temp;
                }
            }

            System.out.println("Monkey business: " + max_1st*max_2nd);

        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }



    }
}