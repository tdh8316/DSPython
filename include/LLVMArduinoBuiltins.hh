#include <Arduino.h>
#include <wiring_private.h>

extern "C" void pin_mode(uint8_t pin, uint8_t mode);
extern "C" float pulse_in(uint8_t pin, uint8_t mode);
extern "C" void digital_write(uint8_t pin, uint8_t val);
extern "C" void analog_write(uint8_t pin, uint8_t val);
extern "C" int digital_read(uint8_t pin);
extern "C" int analog_read(uint8_t pin);
