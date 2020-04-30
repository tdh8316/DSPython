"""
Upload an hex file to Arduino
$ flash.py ARDUINO_DIR HEX_FILE PORT
"""

import os
import sys

ARDUINO_DIR = sys.argv[1]
INPUT = sys.argv[2]
PORT = sys.argv[3]

if not os.path.isdir(ARDUINO_DIR):
    raise FileNotFoundError("Please set your arduino software directory correctly.")

AVRDUDE = "{0}/hardware/tools/avr/bin/avrdude".format(ARDUINO_DIR)

command = (
    "{AVRDUDE} "
    "-C{0}/hardware/tools/avr/etc/avrdude.conf "
    "-v -patmega328p -carduino -P{PORT} -b115200 -D "
    "-Uflash:w:{HEX}:i".format(ARDUINO_DIR, AVRDUDE=AVRDUDE, PORT=PORT, HEX=INPUT,)
)
os.system(command)
