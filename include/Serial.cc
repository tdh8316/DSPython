#include "Serial.h"

extern "C" void print(char n[]) { Serial.println(n); }
extern "C" void begin(int b) { Serial.begin(b); }
