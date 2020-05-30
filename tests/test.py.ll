; ModuleID = 'tests/test.py'
source_filename = "tests/test.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

declare void @pin_mode(i8, i8) local_unnamed_addr addrspace(1)

declare void @begin(i16) local_unnamed_addr addrspace(1)

declare i8 @digital_read(i8) local_unnamed_addr addrspace(1)

declare void @print__i__(i16) local_unnamed_addr addrspace(1)

define void @setup() local_unnamed_addr addrspace(1) {
  tail call addrspace(1) void @begin(i16 9600)
  tail call addrspace(1) void @pin_mode(i8 13, i8 0)
  ret void
}

define void @loop() local_unnamed_addr addrspace(1) {
  %call = tail call addrspace(1) i8 @digital_read(i8 13)
  %icast = sext i8 %call to i16
  tail call addrspace(1) void @print__i__(i16 %icast)
  ret void
}
