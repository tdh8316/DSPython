; ModuleID = 'tests/x.py'
source_filename = "tests/x.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

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
  %count = alloca i16
  store i16 0, i16* %count
  br label %while.cond

while.cond:                                       ; preds = %while.body, %0
  %count1 = load i16, i16* %count
  %a = icmp slt i16 %count1, 10
  br i1 %a, label %while.body, label %while.else

while.body:                                       ; preds = %while.cond
  %count2 = load i16, i16* %count
  tail call addrspace(1) void @print__i__(i16 %count2)
  %count3 = load i16, i16* %count
  %add = add i16 %count3, 1
  store i16 %add, i16* %count
  br i1 %a, label %while.cond, label %while.else

while.else:                                       ; preds = %while.body, %while.cond
  br label %while.after

while.after:                                      ; preds = %while.else
  ret void
}
