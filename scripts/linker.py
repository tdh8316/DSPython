import os
import shutil
import sys

ARDUINO_DIR = sys.argv[1]

CC = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-gcc".format(ARDUINO_DIR=ARDUINO_DIR)
CPP = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-g++".format(ARDUINO_DIR=ARDUINO_DIR)
AR = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-ar".format(ARDUINO_DIR=ARDUINO_DIR)
OBJ_COPY = "{ARDUINO_DIR}/hardware/tools/avr/bin/avr-objcopy".format(
    ARDUINO_DIR=ARDUINO_DIR
)
AVRDUDE = "{0}/hardware/tools/avr/bin/avrdude".format(ARDUINO_DIR)

if not os.path.isdir(ARDUINO_DIR) or shutil.which(CC):
    raise FileNotFoundError("Please set your arduino software directory correctly.")

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

OUT_PREFIX = "arduino_build/"

if not os.path.isdir(OUT_PREFIX):
    try:
        os.makedirs(OUT_PREFIX)
    except Exception as e:
        raise e

compile_commands = """
llc -filetype=obj {INPUT} -o {INPUT}.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}WInterrupts.c -o {OUT_PREFIX}WInterrupts.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring.c -o {OUT_PREFIX}wiring.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_analog.c -o {OUT_PREFIX}wiring_analog.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_digital.c -o {OUT_PREFIX}wiring_digital.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_pulse.c -o {OUT_PREFIX}wiring_pulse.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_shift.c -o {OUT_PREFIX}wiring_shift.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}hooks.c -o {OUT_PREFIX}hooks.c.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}CDC.cpp -o {OUT_PREFIX}CDC.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}HardwareSerial.cpp -o {OUT_PREFIX}HardwareSerial.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}IPAddress.cpp -o {OUT_PREFIX}IPAddress.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}main.cpp -o {OUT_PREFIX}main.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}new.cpp -o {OUT_PREFIX}new.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}Print.cpp -o {OUT_PREFIX}Print.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}Stream.cpp -o {OUT_PREFIX}Stream.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}Tone.cpp -o {OUT_PREFIX}Tone.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}USBCore.cpp -o {OUT_PREFIX}USBCore.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}WMath.cpp -o {OUT_PREFIX}WMath.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}WString.cpp -o {OUT_PREFIX}WString.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}PluggableUSB.cpp -o {OUT_PREFIX}PluggableUSB.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}HardwareSerial0.cpp -o {OUT_PREFIX}HardwareSerial0.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}HardwareSerial3.cpp -o {OUT_PREFIX}HardwareSerial3.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}HardwareSerial2.cpp -o {OUT_PREFIX}HardwareSerial2.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}HardwareSerial1.cpp -o {OUT_PREFIX}HardwareSerial1.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}abi.cpp -o {OUT_PREFIX}abi.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {CWD}/include/Serial.cc -o {OUT_PREFIX}Serial.cc.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}WInterrupts.c.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring.c.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_analog.c.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_digital.c.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_pulse.c.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_shift.c.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}hooks.c.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}CDC.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}IPAddress.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}main.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}new.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Print.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Stream.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Tone.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}USBCore.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}WMath.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}WString.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}PluggableUSB.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial0.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial3.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial2.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial1.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}abi.cpp.o
{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Serial.cc.o
{CC} -w -Os -g -flto -fuse-linker-plugin -Wl,--gc-sections -mmcu={MCU} -o {INPUT}.elf {INPUT}.o {OUT_PREFIX}core.a -lm
{OBJ_COPY} -O ihex -j .eeprom {OBJ} .eeprom=0 {INPUT}.elf {INPUT}.eep
{OBJ_COPY} -O ihex -R .eeprom {INPUT}.elf {INPUT}.hex
""".format(
    CC=CC,
    CPP=CPP,
    AR=AR,
    OBJ_COPY=OBJ_COPY,
    MCU=MCU,
    GENERAL_FLAGS=GENERAL_FLAGS,
    CPP_FLAGS=CPP_FLAGS,
    CC_FLAGS=CC_FLAGS,
    INCLUDE_FILES=INCLUDE_FILES,
    LIBRARY_DIR=LIBRARY_DIR,
    OBJ="--set-section-flags=.eeprom=alloc,load --no-change-warnings --change-section-lma",
    INPUT=sys.argv[2],
    OUT_PREFIX=OUT_PREFIX,
    CWD=os.getcwd(),
)

for command in compile_commands.splitlines():
    code: int = os.system(command)
    if code != 0:
        sys.exit(1)

os.remove("{INPUT}.elf".format(INPUT=sys.argv[2]))
os.remove("{INPUT}.eep".format(INPUT=sys.argv[2]))
os.remove("{INPUT}.o".format(INPUT=sys.argv[2]))
# os.remove("{INPUT}".format(INPUT=sys.argv[2]))
