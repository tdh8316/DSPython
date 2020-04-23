import os
from scripts.env import *


command = (
    "{AVRDUDE} "
    "-C{0}/hardware/tools/avr/etc/avrdude.conf "
    "-v -patmega328p -carduino -P{PORT} -b115200 -D "
    "-Uflash:w:{HEX}:i".format(
        ARDUINO_DIR, AVRDUDE=AVRDUDE, PORT=input("PORT:"), HEX=input("HEX FILE:")
    )
)
os.system(command)
