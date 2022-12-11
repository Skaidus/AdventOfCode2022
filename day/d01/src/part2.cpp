#include <iostream>
#include <fstream>
#include <string>
using namespace std;



int main() {
    ifstream file("../input.txt");
    if(file.is_open()){
        string line;
        int curr = 0, sum = 0, i;
        int top3[3] = { };
        while(getline(file, line)){
            if(line.empty()) {
                for(i = 0; i < 3; i++){
                    if(curr > top3[i]) swap(curr, top3[i]);
                }
                curr = 0;
            } else curr += stoi(line);
        }
        for(i = 0; i < 3; i++) sum += top3[i];
        cout << "Max calories: " << sum;
        file.close();
    }
    return 0;
}
