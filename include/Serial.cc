#include "Serial.hh"

extern "C" void print__i__(int n) { Serial.print(n); }
extern "C" void print__f__(float n) { Serial.print(n); }
extern "C" void print__s__(char c[]) { Serial.print(c); }
extern "C" void println__i__(int n) { Serial.println(n); }
extern "C" void println__f__(float n) { Serial.println(n); }
extern "C" void println__s__(char c[]) { Serial.println(c); }
extern "C" int is_serial_available() { return Serial.available(); }
extern "C" void serial_begin(int b) { Serial.begin(b); }
extern "C" int input() { return Serial.read(); }
extern "C" void flush() { return Serial.flush(); }