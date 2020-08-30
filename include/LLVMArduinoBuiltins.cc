#include "LLVMArduinoBuiltins.h"

extern "C" void pin_mode(uint8_t pin, uint8_t mode) {
    return pinMode(pin, mode);
}

extern "C" void digital_write(uint8_t pin, uint8_t val) {
    return digitalWrite(pin, val);
}

extern "C" int digital_read(uint8_t pin) {
    return digitalRead(pin);
}

extern "C" int analog_read(uint8_t pin) {
    return analogRead(pin);
}
