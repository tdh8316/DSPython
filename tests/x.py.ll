; ModuleID = 'tests/x.py'
source_filename = "tests/x.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

@.str = private unnamed_addr constant [7 x i8] c"Low...\00", align 1
@.str.1 = private unnamed_addr constant [16 x i8] c"Finally High!!!\00", align 1

declare void @pin_mode(i8, i8) addrspace(1)

declare void @serial_begin(i16) addrspace(1)

declare void @delay(i32) addrspace(1)

declare void @digital_write(i8, i8) addrspace(1)

declare i16 @digital_read(i8) addrspace(1)

declare void @print__i__(i16) addrspace(1)

declare void @print__f__(float) addrspace(1)

declare void @print__s__(i8*) addrspace(1)

declare i16 @int__f__(float) addrspace(1)

declare i16 @int__i__(i16) addrspace(1)

declare float @float__f__(float) addrspace(1)

declare float @float__i__(i16) addrspace(1)

define void @setup() addrspace(1) {
  tail call addrspace(1) void @serial_begin(i16 9600)
  tail call addrspace(1) void @pin_mode(i8 9, i8 1)
  ret void
}

define void @loop() addrspace(1) {
  %call = tail call addrspace(1) i16 @digital_read(i8 9)
  %volt = alloca i16
  store i16 %call, i16* %volt
  br label %while.cond

while.cond:                                       ; preds = %while.body, %0
  %volt1 = load i16, i16* %volt
  %a = icmp ne i16 %volt1, 1
  br i1 %a, label %while.body, label %while.else

while.body:                                       ; preds = %while.cond
  tail call addrspace(1) void @print__s__(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.str, i32 0, i32 0))
  br i1 %a, label %while.cond, label %while.else

while.else:                                       ; preds = %while.body, %while.cond
  tail call addrspace(1) void @print__s__(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @.str.1, i32 0, i32 0))
  br label %while.after

while.after:                                      ; preds = %while.else
  ret void
}
