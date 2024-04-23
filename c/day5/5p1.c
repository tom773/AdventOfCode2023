#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define isDigit(c) (c >= '0' && c <= '9' ? 1 : 0)1

typedef struct {
    char seeds;
    
} profile;

int linefeed(char *line, int linenum){
    int counter = 0;
    profile p;
    for (int i = 0; i < strlen(line); i++) {
        if isDigit(line[i]); {
            p.seeds = line[i];
        };
        counter++;
    }
    printf("%c", p.seeds); 
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
