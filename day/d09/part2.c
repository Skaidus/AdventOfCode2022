#include <stdio.h>
#include <stdlib.h>

struct Point {
    int i, j;
};

int main() {
    char dir;
    int amount;
    FILE *f = fopen("input.txt", "r");
    int max_i = 1, min_i = 0;
    int max_j = 1, min_j = 0;
    int curr_i = 0, curr_j = 0;
    while(!feof(f)){
        fscanf(f,"%c %d\n", &dir, &amount);
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
        for (int j = 0; j < size_j; j++) {
            grid[i][j] = 0;
        }
    }
    grid[-min_i][-min_j] = 1;
    struct Point T, H, Hp;
    int sum = 1;
    int di, dj;
    T.i = H.i = -min_i;
    T.j = H.j = -min_j;
    rewind(f);
    int line_count = 0;
    const int KNOTS = 10;
    struct Point rope[KNOTS];
    for(int i = 0; i < KNOTS; i++){
        rope[i].i = -min_i;
        rope[i].j = -min_j;
    }
    while(!feof(f)) {
        line_count++;
        fscanf(f,"%c %d\n", &dir, &amount);
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
        int vi, vj;
        for(int i = 0; i < amount; i++){

            rope[0].i += di; rope[0].j += dj;
            for(int j = 1; j < KNOTS; j++){
                if((abs(rope[j-1].i - rope[j].i) >1) || (abs(rope[j-1].j - rope[j].j)>1)){
                    vi = rope[j-1].i - rope[j].i;
                    vj = rope[j-1].j - rope[j].j;
                    if(abs(vi)==2) vi /=2;
                    if(abs(vj)==2) vj /=2;
                    rope[j].i += vi;
                    rope[j].j += vj;
                    if(j == KNOTS-1){
                        if(grid[rope[KNOTS-1].i][rope[KNOTS-1].j]==0) {
                            grid[rope[KNOTS-1].i][rope[KNOTS-1].j] = 1; sum++;
                        }
                    }
                }
            }
        }
    }
    printf("Visited positions: %d\n", sum);
}
