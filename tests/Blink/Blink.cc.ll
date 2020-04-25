; ModuleID = '../tests/Blink/Blink.cc'
source_filename = "../tests/Blink/Blink.cc"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

; Function Attrs: nounwind optsize
define dso_local void @setup() local_unnamed_addr addrspace(1) #0 {
  tail call addrspace(1) void @pinMode(i8 zeroext 13, i8 zeroext 1) #2
  ret void
}

; Function Attrs: optsize
declare dso_local void @pinMode(i8 zeroext, i8 zeroext) local_unnamed_addr addrspace(1) #1

; Function Attrs: nounwind optsize
define dso_local void @loop() local_unnamed_addr addrspace(1) #0 {
  tail call addrspace(1) void @digitalWrite(i8 zeroext 13, i8 zeroext 1) #2
  tail call addrspace(1) void @delay(i32 1000) #2
  tail call addrspace(1) void @digitalWrite(i8 zeroext 1, i8 zeroext 0) #2
  tail call addrspace(1) void @delay(i32 1000) #2
  ret void
}

; Function Attrs: optsize
declare dso_local void @digitalWrite(i8 zeroext, i8 zeroext) local_unnamed_addr addrspace(1) #1

; Function Attrs: optsize
declare dso_local void @delay(i32) local_unnamed_addr addrspace(1) #1

attributes #0 = { nounwind optsize "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="atmega328p" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { optsize "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="atmega328p" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { nounwind optsize }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"wchar_size", i32 2}
!1 = !{!"clang version 10.0.0 "}
