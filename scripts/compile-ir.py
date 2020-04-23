from scripts.env import *

# TODO: Change output directory
compile_commands = """
llc -filetype=obj {INPUT} -o {INPUT}.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}WInterrupts.c -o WInterrupts.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring.c -o wiring.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_analog.c -o wiring_analog.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_digital.c -o wiring_digital.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_pulse.c -o wiring_pulse.c.o
{CC} {CC_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}wiring_shift.c -o wiring_shift.c.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}CDC.cpp -o CDC.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}HardwareSerial.cpp -o HardwareSerial.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}IPAddress.cpp -o IPAddress.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}main.cpp -o main.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}new.cpp -o new.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}Print.cpp -o Print.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}Stream.cpp -o Stream.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}Tone.cpp -o Tone.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}USBCore.cpp -o USBCore.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}WMath.cpp -o WMath.cpp.o
{CPP} {CPP_FLAGS} {INCLUDE_FILES} {LIBRARY_DIR}WString.cpp -o WString.cpp.o
{AR} rcs core.a WInterrupts.c.o
{AR} rcs core.a wiring.c.o
{AR} rcs core.a wiring_analog.c.o
{AR} rcs core.a wiring_digital.c.o
{AR} rcs core.a wiring_pulse.c.o
{AR} rcs core.a wiring_shift.c.o
{AR} rcs core.a CDC.cpp.o
{AR} rcs core.a HardwareSerial.cpp.o
{AR} rcs core.a IPAddress.cpp.o
{AR} rcs core.a main.cpp.o
{AR} rcs core.a new.cpp.o
{AR} rcs core.a Print.cpp.o
{AR} rcs core.a Stream.cpp.o
{AR} rcs core.a Tone.cpp.o
{AR} rcs core.a USBCore.cpp.o
{AR} rcs core.a WMath.cpp.o
{AR} rcs core.a WString.cpp.o
{CC} -Os -Wl,--gc-sections -mmcu={MCU} -o {INPUT}.elf {INPUT}.o core.a -lm
{OBJ_COPY} -O ihex -j .eeprom --set-section-flags=.eeprom=alloc,load --no-change-warnings --change-section-lma .eeprom=0 {INPUT}.elf {INPUT}.eep
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
    INPUT=input("LLVM IR FILE:"),
)

for command in compile_commands.splitlines():
    os.system(command)
