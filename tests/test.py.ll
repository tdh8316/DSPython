; ModuleID = 'tests/test.py'
source_filename = "tests/test.py"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

declare void @begin(i16) local_unnamed_addr addrspace(1)

declare void @print__f__(float) local_unnamed_addr addrspace(1)

; Function Attrs: norecurse nounwind readnone
define float @add(float %a, float %b) local_unnamed_addr addrspace(1) #0 {
  %add = fadd float %a, %b
  ret float %add
}

define void @setup() local_unnamed_addr addrspace(1) {
  tail call addrspace(1) void @begin(i16 9600)
  %call = tail call addrspace(1) float @add(float 3.000000e+00, float 2.500000e+00)
  tail call addrspace(1) void @print__f__(float %call)
  tail call addrspace(1) void @print__f__(float 0x404987AE20000000)
  ret void
}

; Function Attrs: norecurse nounwind readnone
define void @loop() local_unnamed_addr addrspace(1) #0 {
  ret void
}

attributes #0 = { norecurse nounwind readnone }
