// The stdio.h header defines three variable types, several macros, and various functions for performing input and output operations.
#include <stdio.h>

int main() {
    printf("Hello, World!\n");

    // Variables
    int x;
    int y = 10;
    y = 100;
    char firstInitial = 'P';

    // Conversion characters
    printf("The value of the %s y is %d.\n", "variable", y); 

    // Input using scanf
    int height;
    printf("How tall are you?\n");
    scanf("%d", &height);
    printf("You are %d cm tall.\n", height);

    // Character arrays
    char name[20];
    printf("What is your name?\n");
    scanf("%19s", name);
    printf("Your name is %s.\n", name);

    return 0;
}

// Compile with the command gcc cpt109-basics.c, then run the executable ./a.out