#include <stdio.h>
#include <stdarg.h>

static char inputBuffer[0xFF + 1];

void print_v(void)
{
    printf("\n");
    fflush(stdout);
}

void print_i(int value)
{
    printf("%d\n", value);
    fflush(stdout);
}

void print_f(float value)
{
    printf("%f\n", value);
    fflush(stdout);
}

void print_p(int args, ...)
{
    va_list ap;
    va_start(ap, args);
    for (int i = 0; i < args; ++i)
    {
        char *string = va_arg(ap, char *);
        printf("%s ", string);
    }
    va_end(ap);
    printf("\n");
    fflush(stdout);
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
