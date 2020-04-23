import os

ARDUINO_DIR = (
    "D:/arduino-1.8.12"
    if "ARDUINO_ROOT" not in os.environ
    else os.environ["ARDUINO_ROOT"]
)
if not os.path.isdir(ARDUINO_DIR):
    raise FileNotFoundError("Please set your arduino software directory correctly.")

CC = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-gcc".format(ARDUINO_DIR=ARDUINO_DIR)
CPP = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-g++".format(ARDUINO_DIR=ARDUINO_DIR)
AR = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-ar".format(ARDUINO_DIR=ARDUINO_DIR)
OBJ_COPY = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-objcopy".format(
    ARDUINO_DIR=ARDUINO_DIR
)
AVRDUDE = "{0}/hardware/tools/avr/bin/avrdude".format(ARDUINO_DIR)

F_CPU = 16000000
MCU = "atmega328p"

GENERAL_FLAGS = (
    "-c -g -Os -Wall -ffunction-sections -fdata-sections "
    "-mmcu={MCU} -DF_CPU={F_CPU}L -MMD -DUSB_VID=null -DUSB_PID=null -DARDUINO=106".format(
        F_CPU=F_CPU, MCU=MCU
    )
)
CPP_FLAGS = "{} -fno-exceptions".format(GENERAL_FLAGS)
CC_FLAGS = GENERAL_FLAGS

# location of include files
INCLUDE_FILES = (
    '"-I{0}/hardware/arduino/avr/cores/arduino" '
    '"-I{0}/hardware/arduino/avr/variants/standard" '
    '"-I{0}/hardware/tools/avr/avr/include"'.format(ARDUINO_DIR)
)
# library sources
LIBRARY_DIR = "{}/hardware/arduino/avr/cores/arduino/".format(ARDUINO_DIR)
