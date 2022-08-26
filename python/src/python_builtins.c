#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

static int n;

int int_s(const char *value)
{
    const char *chars = strdup(value);
    while (*chars)
    {
        if (!isdigit(*chars++))
        {
            printf("ValueError: Invalid literal for int() with base 10: '%s'\n", value);
            exit(1);
        }
    }

    n = atoi(value);
    return n;
}

int int_f(const float value)
{
    n = floor(value);
    return n;
}
