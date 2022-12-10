#include <stdio.h>
#include <stdlib.h>

struct Point {
    int i, j;
};

int main() {
    char line[4];
    char dir;
    int amount;
    FILE *f = fopen("input.txt", "r");
    int max_i = 1, min_i = 0;
    int max_j = 1, min_j = 0;
    int curr_i = 0, curr_j = 0;
    while((fgets(line, 8, f)) != NULL) {
        sscanf(line, "%c %d", &dir, &amount);
        switch (dir) {
            case 'U':
                curr_i+=amount;
                if(curr_i >= max_i) max_i = curr_i;
                break;
            case 'D':
                curr_i-=amount;
                if(curr_i <= min_i) min_i = curr_i;
                break;
            case 'L':
                curr_j-=amount;
                if(curr_j <= min_j) min_j = curr_j;
                break;
            default:
                curr_j+=amount;
                if(curr_j >= max_j) max_j = curr_j;
                break;
        }
    }
    const int size_i = max_i - min_i + 2;
    const int size_j = max_j - min_j + 2;
    int grid[size_i][size_j];
    for (int i = 0; i < size_i; i++) {
        for (int j = 0; j < size_j; j++) grid[i][j] = 0;
    }
    grid[-min_i][-min_j] = 1;
    struct Point T, H, Hp;
    int sum = 1;
    int di, dj;
    T.i = H.i = -min_i;
    T.j = H.j = -min_j;
    rewind(f);
    int line_count = 0;
    while((fgets(line, 8, f)) != NULL) {
        line_count++;
        sscanf(line, "%c %d", &dir, &amount);
        switch (dir) {
            case 'U':
                di = 1; dj = 0;
                break;
            case 'D':
                di = -1; dj = 0;
                break;
            case 'L':
                di = 0; dj = -1;
                break;
            default:
                di = 0; dj = 1;
                break;
        }
        for(int i = 0; i < amount; i++){
            Hp.i = H.i + di; Hp.j = H.j + dj;
            if((abs(Hp.i - T.i) >1) || (abs(Hp.j - T.j)>1)){
                T.i = Hp.i; T.j = Hp.j;
                if(grid[T.i][T.j]==0) {
                    grid[T.i][T.j] = 1; sum++;
                }
            }
            H.i = Hp.i; H.j = Hp.j;
        }
    }
    printf("Visited positions: %d\n", sum);
}
