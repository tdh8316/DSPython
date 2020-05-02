#include "Serial.h"

extern "C" void printi(int n) { Serial.println(n); }
extern "C" void prints(char c[]) { Serial.println(c); }
extern "C" void begin(int b) { Serial.begin(b); }
