#include <iostream>
#include <fstream>
#include <string>
using namespace std;

int main() {
    ifstream file("../input.txt");
    if(file.is_open()){
        string line;
        int curr = 0, max = 0;
        while(getline(file, line)){
            if(line.empty()) {
                if(curr > max) max = curr;
                curr = 0;
            } else curr += stoi(line);
        }
        cout << "Max calories: " << max;
        file.close();
    }
    return 0;
}
