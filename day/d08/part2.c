#include <stdio.h>

struct Tree {
    char height;
    int score;
};

int const grid_size = 99;

int main() {
    int max = 0, n_line = 0;
    char line[grid_size];
    struct Tree grid[grid_size][grid_size];
    FILE *f = fopen("./input.txt", "r");

    while((fgets(line, grid_size+3, f)) != NULL) {
        for(int i = 0; i < grid_size; i++) {
            grid[n_line][i].height = line[i];
            grid[n_line][i].score = 1;
        }
        for(int i = 1; i < grid_size-1; i++) {
            int local_score = 0;
            for(int j = i-1; j >= 0; j--){
                local_score++;
                if(grid[n_line][j].height >= grid[n_line][i].height) break;
            }
            grid[n_line][i].score *= local_score;
        }
        for(int i = grid_size - 2; i > 0; i--) {
            int local_score = 0;
            for(int j = i+1; j < grid_size; j++){
                local_score++;
                if(grid[n_line][j].height >= grid[n_line][i].height) break;
            }
            grid[n_line][i].score *= local_score;
        }

        n_line++;
    }

    for(int k = 1; k < grid_size-1; k++) {
        for(int i = 1; i < grid_size-1; i++) {
            int local_score = 0;
            for(int j = i-1; j >= 0; j--){
                local_score++;
                if(grid[j][k].height >= grid[i][k].height) break;
            }
            grid[i][k].score *= local_score;
        }
        for(int i = grid_size - 2; i > 0; i--) {
            int local_score = 0;
            for(int j = i+1; j < grid_size; j++){
                local_score++;
                if(grid[j][k].height >= grid[i][k].height) break;
            }
            grid[i][k].score *= local_score;
            if(grid[i][k].score>max) max = grid[i][k].score;
        }
    }

    fclose(f);
    printf("Max visibility: %d\n", max);
    return 0;
}
