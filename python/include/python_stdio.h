#ifndef PYTHON_STDIO
#define PYTHON_STDIO

void print_v(void);
void print_i(int value);
void print_f(float value);
void print_p(int args, ...);

char *input_v(void);
char *input_p(char *value);

#endif // PYTHON_STDIO
