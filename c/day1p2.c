#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

// This was, for some reason, the hardest AOC for me yet. Despite it being a part 1
// Probably because I'm using C. 2 hours of Segmentation Faults later, and this works!
// 40 lines of code for a simple task. I'm not proud of this one.

int total = 0;

int linefeed(char *line, int linenum){
    for(int i = 12; i <= 20; i++){
        char c_arr[100];
        int n = i;
        sprintf(c_arr, "%d", i);
        char *w_arr = strstr(line, c_arr);
    
        if (w_arr != NULL) {
            char *w_arr = strstr(line, c_arr);
            char color = w_arr[3];
            if ((color == 'r' && n > 12) ||
                (color == 'g' && n > 13) ||
                (color == 'b' && n > 14)) {
                return 1;
            }         
        }
    }
    total+=linenum;
    return 0;
} 

int main() {
    
    FILE *file;
    char buffer[1024] = {};

    int ln = 1;
    file = fopen("../inputs/2p1.txt", "r");
    while (fgets(buffer, sizeof(buffer), file) != NULL) {
        linefeed(buffer, ln);
        ln++;
    }
    printf("%d\n", total);   
    fclose(file);
}
