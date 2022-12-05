import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class OrderParser {
    private static final Pattern order_parser = Pattern.compile("^move ([0-9]+) from ([0-9]+) to ([0-9]+)");
    public int blocks, from, to;

    public void parseOrder(String data){
        Matcher m = order_parser.matcher(data);
        if (m.find()){
            blocks = Integer.parseInt(m.group(1));
            from = Integer.parseInt(m.group(2));
            to =  Integer.parseInt(m.group(3));
        }
    }
}
