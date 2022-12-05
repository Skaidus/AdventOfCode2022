import java.io.File;  // Import the File class
import java.io.FileNotFoundException;  // Import this class to handle errors
import java.util.Scanner; // Import the Scanner class to read text files
import java.util.Stack;


public class Part1 {

    public static void main(String[] args) {
        try {
            File myObj = new File("../input.txt");
            Scanner myReader = new Scanner(myObj);
            Stack<String> buffer =  new Stack<>();
            int n_stacks = 0;
            while (myReader.hasNextLine()) {
                String data = myReader.nextLine();
                if(data.charAt(0)!='['){
                    int i = data.length() - 2;
                    while(data.charAt(i) != ' ') i--;
                    String num = data.substring(i+1, i+2);
                    n_stacks = Integer.parseInt(num);
                    break;
                }
                buffer.push(data);
            }
            ICraneOperator operator = new CraneOperator(n_stacks);
            while(!buffer.isEmpty()){
                String line = buffer.pop();
                for(int i = 1, j = 0; i < line.length()-1; i+=4, j++){
                    char crate = line.charAt(i);
                    if(crate != ' ') operator.addCrate(crate, j);
                }
            }
            OrderParser p = new OrderParser();
            myReader.nextLine();
            while (myReader.hasNextLine()) {
                String data = myReader.nextLine();
                p.parseOrder(data);
                operator.operate(p.blocks, p.from-1, p.to-1);
            }
            myReader.close();
            System.out.println(operator.getTop());
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }
}