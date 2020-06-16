#include <Arduino.h>
#include "Print.h"
#include <HardwareSerial.h>

extern "C" void print__i__(int n);
extern "C" void print__f__(float n);
extern "C" void print__s__(char c[]);
extern "C" void serial_begin(int b);
