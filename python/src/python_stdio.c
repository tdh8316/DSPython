#include <stdio.h>

static char inputBuffer[0xFF + 1];

void print_v(void)
{
    printf("\n");
}

void print_i(int value)
{
    printf("%d\n", value);
}

void print_f(float value)
{
    printf("%f\n", value);
}

void print_p(char *value)
{
    printf("%s\n", value);
}

char *input_v(void)
{
    fgets(inputBuffer, sizeof(inputBuffer), stdin);

    // remove the trailing newline
    inputBuffer[strcspn(inputBuffer, "\n")] = 0;
    inputBuffer[strcspn(inputBuffer, "\r\n")] = 0;

    return inputBuffer;
}

char *input_p(char *value)
{
    printf("%s", value);
    fflush(stdout);
    fgets(inputBuffer, sizeof(inputBuffer), stdin);

    // remove the trailing newline
    inputBuffer[strcspn(inputBuffer, "\n")] = 0;
    inputBuffer[strcspn(inputBuffer, "\r\n")] = 0;

    return inputBuffer;
}
