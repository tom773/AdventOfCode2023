#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>

int total = 0;

void stripstring(char *str) {

    char *s = str;
    char *e = str + strlen(str) - 1;

    while (*s && isspace(*s)) s++;

    while (e > s && isspace(*e)) e--;

    size_t len = e - s + 1;

    memmove(str, s, len);

    str[len] = '\0';

}

void splitstr(char *line){
    
    char *balls[256];
    int numballs = 0;
    
    int redhighest = 0;
    int bluehighest = 0;
    int greenhighest = 0;

    char delims[] = ";,";
    char *token = strtok(line, delims);

    while (token != NULL) {
        stripstring(token);
        balls[numballs++] = token;
        token = strtok(NULL, delims);
    }

    char *temp = strstr(balls[0], ": ");
    balls[0] = &temp[2];

    for (int i = 0; i < numballs; i++) {
        int number;
        if (strstr(balls[i], "red") != NULL) {
            if (sscanf(balls[i], "%d", &number) == 1){
                if (number > redhighest) {
                    redhighest = number;
                }
            } 
        }
        if (strstr(balls[i], "blue") != NULL) {
            if (sscanf(balls[i], "%d", &number) == 1){
                if (number > bluehighest) {
                    bluehighest = number;
                }
            } 
        }
        if (strstr(balls[i], "green") != NULL) {
            if (sscanf(balls[i], "%d", &number) == 1){
                if (number > greenhighest) {
                    greenhighest = number;
                }
            } 
        }
    }
    total += redhighest*bluehighest*greenhighest;
}

int main() {
    
    FILE *file;
    char buffer[1024] = {};

    int ln = 0;
    file = fopen("../inputs/2p2.txt", "r");
    while (fgets(buffer, sizeof(buffer), file) != NULL) {
        ln++;
        splitstr(buffer);
    }
    printf("%d\n", total);
    fclose(file);
}
