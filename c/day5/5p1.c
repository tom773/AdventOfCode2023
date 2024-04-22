#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

int total = 0;

int linefeed(char *line, int linenum){
    printf("%s", line); 
    return 0;
} 

int main() {
    
    FILE *file;
    char buffer[1024];

    int ln = 1;
    file = fopen("./5p1.txt", "r");
    while (fgets(buffer, 1024, file) != NULL) {
        linefeed(buffer, ln);
        ln++;
    }
    fclose(file);
}
