; ModuleID = '../tests/SoftwareSerialExample/SoftwareSerialExample.cc'
source_filename = "../tests/SoftwareSerialExample/SoftwareSerialExample.cc"
target datalayout = "e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8"
target triple = "avr"

%class.HardwareSerial = type { %class.Stream, i8*, i8*, i8*, i8*, i8*, i8*, i8, i8, i8, i8, i8, [64 x i8], [64 x i8] }
%class.Stream = type { %class.Print, i32, i32 }
%class.Print = type { i32 (...)**, i16 }

@Serial = external dso_local global %class.HardwareSerial, align 1
@.str = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1

; Function Attrs: nounwind optsize
define dso_local void @setup() local_unnamed_addr addrspace(1) #0 {
  tail call addrspace(1) void @_ZN14HardwareSerial5beginEmh(%class.HardwareSerial* nonnull @Serial, i32 9600, i8 zeroext 6) #3
  %1 = tail call addrspace(1) i16 @_ZN5Print7printlnEPKc(%class.Print* getelementptr inbounds (%class.HardwareSerial, %class.HardwareSerial* @Serial, i16 0, i32 0, i32 0), i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str, i16 0, i16 0)) #3
  ret void
}

; Function Attrs: optsize
declare dso_local i16 @_ZN5Print7printlnEPKc(%class.Print*, i8*) local_unnamed_addr addrspace(1) #1

; Function Attrs: norecurse nounwind optsize readnone
define dso_local void @loop() local_unnamed_addr addrspace(1) #2 {
  ret void
}

; Function Attrs: optsize
declare dso_local void @_ZN14HardwareSerial5beginEmh(%class.HardwareSerial*, i32, i8 zeroext) local_unnamed_addr addrspace(1) #1

attributes #0 = { nounwind optsize "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="atmega328p" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { optsize "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="atmega328p" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { norecurse nounwind optsize readnone "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="atmega328p" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nounwind optsize }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"wchar_size", i32 2}
!1 = !{!"clang version 10.0.0 "}
