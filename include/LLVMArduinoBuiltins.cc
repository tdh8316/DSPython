#include "LLVMArduinoBuiltins.hh"

extern "C" void pin_mode(uint8_t pin, uint8_t mode) {
    return pinMode(pin, mode);
}

extern "C" float pulse_in(uint8_t pin, uint8_t mode) {
    return pulseIn(pin, mode);
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
