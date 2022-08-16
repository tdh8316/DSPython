#include <stdlib.h>
#include <math.h>

static int n;

int int_s(const char *value)
{
    n = atoi(value);
    return n;
}

int int_f(const float value)
{
    n = floor(value);
    return n;
}
