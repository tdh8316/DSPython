; ModuleID = 'tests/x.py'
source_filename = "tests/x.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

@.str = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1
@.str.1 = private unnamed_addr constant [32 x i8] c"While loop in setup function!!!\00", align 1
@.str.2 = private unnamed_addr constant [21 x i8] c"Out of while loop!!!\00", align 1

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
  tail call addrspace(1) void @delay(i32 1000)
  br i1 true, label %then, label %else

then:                                             ; preds = %0
  tail call addrspace(1) void @print__s__(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str, i32 0, i32 0))
  br label %cont

else:                                             ; preds = %0
  br label %cont

cont:                                             ; preds = %else, %then
  %a = alloca i16
  store i16 1, i16* %a
  br label %while

while.body:                                       ; preds = %while
  tail call addrspace(1) void @print__s__(i8* getelementptr inbounds ([32 x i8], [32 x i8]* @.str.1, i32 0, i32 0))
  %a3 = load i16, i16* %a
  %add = add i16 %a3, 1
  store i16 %add, i16* %a
  tail call addrspace(1) void @delay(i32 1000)
  br i1 %a2, label %while, label %while.else

while.else:                                       ; preds = %while.body, %while
  tail call addrspace(1) void @print__s__(i8* getelementptr inbounds ([21 x i8], [21 x i8]* @.str.2, i32 0, i32 0))
  br label %while.after

while.after:                                      ; preds = %while.else
  ret void

while:                                            ; preds = %while.body, %cont
  %a1 = load i16, i16* %a
  %a2 = icmp eq i16 %a1, 1
  br i1 %a2, label %while.body, label %while.else
}

define void @loop() addrspace(1) {
  ret void
}
