; ModuleID = 'tests\LED\test.py'
source_filename = "tests\\LED\\test.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

@foo = unnamed_addr global i16 6974

declare void @pinMode(i8, i8) addrspace(1)

declare void @delay(i32) addrspace(1)

declare void @digitalWrite(i8, i8) addrspace(1)

define void @setup() addrspace(1) {
  %a = alloca i16
  store i16 2, i16* %a
  %a1 = load i16, i16* %a
  %foo = load i16, i16* @foo
  %add = add i16 %a1, %foo
  %b = alloca i16
  store i16 %add, i16* %b
  %b2 = load i16, i16* %b
  %sub = sub i16 %b2, 6963
  %i8 = trunc i16 %sub to i8
  tail call addrspace(1) void @pinMode(i8 %i8, i8 1)
  ret void
}

define void @loop() addrspace(1) {
  tail call addrspace(1) void @digitalWrite(i8 13, i8 1)
  ret void
}
