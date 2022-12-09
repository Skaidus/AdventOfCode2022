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
    int max_i = 1, max_j = 1;
    while((fgets(line, 6, f)) != NULL) {
        sscanf(line, "%c %d", &dir, &amount);
        switch (dir) {
            case 'U':
                max_i++;
                break;
            case 'D':
                max_i--;
                break;
            case 'L':
                max_j--;
                break;
            default:
                max_j++;
                break;
        }
    }
    int grid[max_i][max_j];
    for (int i = 0; i < max_i; i++) {
        for (int j = 0; j < max_j; j++) grid[i][j] = 0;
    }
    grid[0][0] = 1;
    struct Point T, H, Hp;
    int sum = 1;
    int di, dj;
    T.i = T.j = H.i = H.j = 0;
    while((fgets(line, 6, f)) != NULL) {
        sscanf(line, "%c %d", dir, amount);
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
