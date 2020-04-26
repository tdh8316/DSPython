#include "Serial.h"

extern "C" void print(int n) { Serial.println(n); }
extern "C" void begin(int b) { Serial.begin(b); }
