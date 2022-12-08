#include <stdio.h>

struct Tree {
    char height;
    int seen;
};

int const grid_size = 99;

int main() {
    int sum = 0, n_line = 0;
    int i;
    char line[grid_size+1];
    struct Tree grid[grid_size][grid_size];
    FILE *f = fopen("./input.txt", "r");
    char max_height;
    while((fgets(line, grid_size+1, f)) != NULL) {
        for(i = 0; i < grid_size; i++) {
            grid[n_line][i].height = line[i];
            grid[n_line][i].seen = 0;
        }
        max_height = '0';
        for(i = 0; i < grid_size; i++) {
            if(grid[n_line][i].height > max_height) {
                max_height = grid[n_line][i].height;
                if(grid[n_line][i].seen == 0) {
                    grid[n_line][i].seen = 1;
                    sum ++;
                }
            }
        }
        max_height = '0';
        for(i = grid_size-1; i >= 0; i--) {
            if(grid[n_line][i].height > max_height) {
                max_height = grid[n_line][i].height;
                if(grid[n_line][i].seen == 0) {
                    grid[n_line][i].seen = 1;
                    sum ++;
                }
            }
        }
    }
    int j;
    for(j = 0; j < grid_size; j++) {
        max_height = '0';
        for(i = 0; i < grid_size; i++) {
            if(grid[i][j].height > max_height) {
                max_height = grid[i][j].height;
                if(grid[i][j].seen == 0) {
                    grid[i][j].seen = 1;
                    sum ++;
                }
            }
        }
        max_height = '0';
        for(i = grid_size-1; i >= 0; i--) {
            if(grid[i][j].height > max_height) {
                max_height = grid[i][j].height;
                if(grid[i][j].seen == 0) {
                    grid[i][j].seen = 1;
                    sum ++;
                }
            }
        }
    }

    fclose(f);
    printf("Total: %d\n", sum);
    return 0;
}
