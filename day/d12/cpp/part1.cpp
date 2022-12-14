#include <iostream>
#include <queue>

struct cStats {
    int f, g;
};

struct Cell {
    int i,j;
};

struct OrderedCell {
    int i,j,f;
};

inline bool operator <(const OrderedCell &a, const OrderedCell &b){
    return a.f > b.f;
}



using namespace std;

const int COL = 132;
const int ROW = 41;

int aStarSearch(char grid[][COL], Cell start, Cell dest){
    bool closedList[ROW][COL];
    for(auto & i : closedList){
        for(bool & j : i) {
            j = false;
        }
    }
    cStats cellDetails[ROW][COL];
    int i, j;
    for(i = 0; i < ROW; i++){
        for(j = 0; j < COL; j++) {
            cellDetails[i][j].f = INT_MAX;
            cellDetails[i][j].g = INT_MAX;
        }
    }
    i = start.i; j = start.j;
    cellDetails[i][j].f = 0;
    cellDetails[i][j].g = 0;
    priority_queue<OrderedCell> openList;
    openList.push({i,j,0});
    const Cell dirs[4] = {
            {1,0}, // Up
            {-1,0}, // Down
            {0, 1}, // Right
            {0, -1} // Left
    };

    while(!openList.empty()){
        OrderedCell p = openList.top();
        openList.pop();
        i = p.i; j = p.j;
        closedList[i][j] = true;
        if((i == dest.i) && (j == dest.j)) return p.f;
        for(auto v: dirs){
            int ip, jp, gp, hp, fp;
            ip = i + v.i;
            jp = j + v.j;
            if((ip >= 0) && (ip < ROW) && (jp >= 0) &&
            (jp < COL) && ( grid[ip][jp] - grid[i][j] <= 1)
            && (!closedList[ip][jp])){
                gp = cellDetails[i][j].g + 1;
                hp =  abs(dest.i - ip) + abs(dest.j - jp);
                fp = hp + gp;
                if((cellDetails[ip][jp].f == INT_MAX) || (cellDetails[ip][jp].f > fp)){
                    openList.push({ip,jp,fp});
                    cellDetails[ip][jp].f = fp;
                    cellDetails[ip][jp].g = gp;
                }
            }
        }
    }
    return -1;
}



#include <fstream>
#include <string>
#include <cstring>
#include <algorithm>

int main() {
    ifstream file("input.txt");
    if(file.is_open()){
        string line;
        char grid[ROW][COL];
        int i = 0;
        int oi, oj, di, dj;
        while(getline(file, line)){
            for(int j = 0; j < line.size(); j++){
                if(line[j] == 'S') {
                    oi = i; oj = j;
                    line[j] = 'a';
                }
                else if(line[j] == 'E') {
                    di = i; dj = j;
                    line[j] = 'z';
                }
            }
            strcpy(grid[i], line.c_str());
            i++;
        }
        cout << "Shortest path: " << aStarSearch(grid, Cell{oi, oj}, Cell{di, dj});
        file.close();
    }
    return 0;
}

