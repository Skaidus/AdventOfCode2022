#include <stdio.h>
#include<stdlib.h>

struct Tree {
    char height;
    unsigned short int seen;
};

inline void check_tree(const unsigned int i, const unsigned int j,  struct Tree trees[]){

}

int main() {
    unsigned int sum = 0, n_line = 0, i = 0;
    char line[99];
    struct Tree grid[99][99];
    FILE *f = fopen("input.txt", "r");
    char max_height;
    while((fgets(line, 99, f)) != NULL) {
        for(i = 0; i < 99; i++) {
            grid[n_line][i].height = line[i];
            grid[n_line][i].seen = 0;
        }
        max_height = '0';
        for(i = 0; i < 99; i++) {
            if(grid[n_line][i].height > max_height) {
                sum ++;
                max_height = grid[n_line][i].height;
                if(grid[n_line][i].seen == 0) grid[n_line][i].seen = 1;
            }
        }
        for(i = 0; i < 99; i++) {
            if(grid[n_line][i].height > max_height) {
                sum ++;
                max_height = grid[n_line][i].height;
                if(grid[n_line][i].seen == 0) grid[n_line][i].seen = 1;
            }
        }
        sscanf(line, "%d-%d,%d-%d",&i1.begin, &i1.end, &i2.begin, &i2.end);
        if(contains(&i1, &i2)) sum++;
    }
    fclose(f);
    printf("Total: %d\n", sum);
    return 0;
}
