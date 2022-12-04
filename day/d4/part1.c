#include <stdio.h>

struct Interval {
    int begin, end;
};

int contains(struct Interval *i1,struct Interval *i2){
    return ((i1->begin <= i2->begin) && (i2->end <= i1->end)) || ((i2->begin <= i1->begin) && (i1->end <= i2->end));
}

int main() {
    // sum = 0, open file, read line,
    int sum = 0;
    char line[12];
    struct Interval i1, i2;
    FILE *f = fopen("input.txt", "r");
    while((fgets(line, 121, f)) != NULL) {
        sscanf(line, "%d-%d,%d-%d",&i1.begin, &i1.end, &i2.begin, &i2.end);
        if(contains(&i1, &i2)) {
            sum++;
        }
    }
    fclose(f);
    printf("Total: %d\n", sum);
    return 0;
}
