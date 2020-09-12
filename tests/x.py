from arduino import *

button_pin = 2
led_pin = 13


def setup() -> None:
    pin_mode(button_pin, INPUT)
    pin_mode(led_pin, OUTPUT)

    return None


def loop() -> None:
    button_state = digital_read(button_pin)

    if button_state == HIGH:
        digital_write(led_pin, HIGH)
    else:
        digital_write(led_pin, LOW)

    return None
