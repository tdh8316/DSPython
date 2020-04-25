; ModuleID = 'tests\Blink\test.py'
source_filename = "tests\\Blink\\test.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

declare void @pinMode(i8, i8) addrspace(1)

declare void @delay(i32) addrspace(1)

declare void @digitalWrite(i8, i8) addrspace(1)

define void @setup() addrspace(1) {
  tail call addrspace(1) void @pinMode(i8 13, i8 1)
  ret void
}

define void @loop() addrspace(1) {
  tail call addrspace(1) void @digitalWrite(i8 13, i8 1)
  tail call addrspace(1) void @delay(i32 1000)
  tail call addrspace(1) void @digitalWrite(i8 13, i8 0)
  tail call addrspace(1) void @delay(i32 1000)
  ret void
}
