#include <stdio.h>
#include <stdarg.h>

static char inputBuffer[0xFF + 1];

void print(const char *types, ...)
{
    const int args = sizeof(types);
    va_list ap;
    va_start(ap, types);
    while (*types != '\0')
    {
        switch (*types++)
        {
        case 's':
            printf("%s ", va_arg(ap, const char *));
            break;
        case 'i':
            printf("%d ", va_arg(ap, const int));
            break;
        case 'f':
            printf("%f ", va_arg(ap, const double));
            break;
        }
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

char *input_s(char *value)
{
    printf("%s", value);
    fflush(stdout);
    fgets(inputBuffer, sizeof(inputBuffer), stdin);

    // remove the trailing newline
    inputBuffer[strcspn(inputBuffer, "\n")] = 0;
    inputBuffer[strcspn(inputBuffer, "\r\n")] = 0;

    return inputBuffer;
}
