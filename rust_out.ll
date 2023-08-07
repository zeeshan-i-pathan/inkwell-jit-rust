; ModuleID = 'rust_out.f07470f7e9a9dde1-cgu.0'
source_filename = "rust_out.f07470f7e9a9dde1-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define noundef signext i16 @add(i16 noundef signext %a, i16 noundef signext %b) unnamed_addr #0 {
start:
  %0 = add i16 %b, %a
  ret i16 %0
}

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
