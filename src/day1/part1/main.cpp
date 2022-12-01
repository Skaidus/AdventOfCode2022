#include <iostream>
#include <fstream>
#include <string>
using namespace std;

int main() {
    string line;

    int curr = 0, max = 0;
    while(getline(cin, line)){
        if(line.empty()) {
            if(curr > max) max = curr;
            curr = 0;
        } else curr += stoi(line);
    }
    cout << "Max calories: " << max;
    return 0;
}
