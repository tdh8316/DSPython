#include <Arduino.h>
#include "Print.h"
#include <HardwareSerial.h>

extern "C" void printi(int n);
extern "C" void prints(char c[]);
extern "C" void begin(int b);
