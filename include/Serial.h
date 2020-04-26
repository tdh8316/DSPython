#include <Arduino.h>
#include "Print.h"
#include <HardwareSerial.h>

extern "C" void print(int n);
extern "C" void begin(int b);
