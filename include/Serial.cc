#include "Serial.h"

extern "C" void print__i__(int n) { Serial.println(n); }
extern "C" void print__f__(float n) { Serial.println(n); }
extern "C" void print__s__(char c[]) { Serial.println(c); }
extern "C" void begin(int b) { Serial.begin(b); }
