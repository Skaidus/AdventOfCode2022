#include <stdio.h>

struct Interval {
    int begin, end;
};

int min(int a, int b){
    if(a<b) return a;
    else return b;
}

int max(int a, int b){
    if(a>b) return a;
    else return b;
}


int overlaps(struct Interval *i1,struct Interval *i2){
    return max(i1->begin, i2->begin) <= min(i1->end, i2->end);
}

int main() {
    // sum = 0, open file, read line,
    int sum = 0;
    char line[12];
    struct Interval i1, i2;
    FILE *f = fopen("input.txt", "r");
    while((fgets(line, 121, f)) != NULL) {
        sscanf(line, "%d-%d,%d-%d",&i1.begin, &i1.end, &i2.begin, &i2.end);
        if(overlaps(&i1, &i2)) {
            sum++;
        }
    }
    fclose(f);
    printf("Total: %d\n", sum);
    return 0;
}
