; ModuleID = 'tests/x.py'
source_filename = "tests/x.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

declare void @pin_mode(i8, i8) local_unnamed_addr addrspace(1)

declare void @serial_begin(i16) local_unnamed_addr addrspace(1)

declare void @print__i__(i16) local_unnamed_addr addrspace(1)

define void @setup() local_unnamed_addr addrspace(1) {
  tail call addrspace(1) void @serial_begin(i16 9600)
  tail call addrspace(1) void @pin_mode(i8 9, i8 1)
  ret void
}

define void @loop() local_unnamed_addr addrspace(1) {
while.body:
  tail call addrspace(1) void @print__i__(i16 0)
  tail call addrspace(1) void @print__i__(i16 1)
  tail call addrspace(1) void @print__i__(i16 2)
  tail call addrspace(1) void @print__i__(i16 3)
  tail call addrspace(1) void @print__i__(i16 4)
  tail call addrspace(1) void @print__i__(i16 5)
  tail call addrspace(1) void @print__i__(i16 6)
  tail call addrspace(1) void @print__i__(i16 7)
  tail call addrspace(1) void @print__i__(i16 8)
  tail call addrspace(1) void @print__i__(i16 9)
  ret void
}
