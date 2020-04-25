#include <Arduino.h>

void setup() {
    pinMode(13, 1);
}

void loop() {
    digitalWrite(13, 1);
    delay(1000);
    digitalWrite(1, 0);
    delay(1000);
}
