; ModuleID = 'probe5.491bf351-cgu.0'
source_filename = "probe5.491bf351-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }
%"core::ptr::metadata::PtrRepr<[&str]>" = type { [2 x i64] }
%"core::ptr::metadata::PtrRepr<[core::fmt::ArgumentV1<'_>]>" = type { [2 x i64] }
%"core::panic::panic_info::PanicInfo<'_>" = type { { ptr, ptr }, ptr, ptr, i8, [7 x i8] }

@alloc13 = private unnamed_addr constant <{ [116 x i8] }> <{ [116 x i8] c"/Users/ulloacastillo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/mod.rs" }>, align 1
@alloc12 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc13, [16 x i8] c"t\00\00\00\00\00\00\00\8B\01\00\008\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc5 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc6 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc5, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc3 = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc14 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc13, [16 x i8] c"t\00\00\00\00\00\00\00\8C\01\00\00\0D\00\00\00" }>, align 8
@alloc15 = private unnamed_addr constant <{ [116 x i8] }> <{ [116 x i8] c"/Users/ulloacastillo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/num/mod.rs" }>, align 1
@alloc16 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc15, [16 x i8] c"t\00\00\00\00\00\00\00\99\03\00\00\05\00\00\00" }>, align 8
@str.1 = internal constant [25 x i8] c"attempt to divide by zero"
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr88drop_in_place$LT$core..panic..panic_info..PanicInfo..internal_constructor..NoPayload$GT$17h8bbeef3baa2dccfaE", [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h68bf95a9e6cbf8a1E" }>, align 8

; <T as core::any::Any>::type_id
; Function Attrs: uwtable
define i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h68bf95a9e6cbf8a1E"(ptr align 1 %self) unnamed_addr #0 {
start:
; call core::any::TypeId::of
  %0 = call i64 @_ZN4core3any6TypeId2of17h6b917d2f7d2f99f6E()
  ret i64 %0
}

; core::any::TypeId::of
; Function Attrs: uwtable
define i64 @_ZN4core3any6TypeId2of17h6b917d2f7d2f99f6E() unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  store i64 -1399122647779529316, ptr %0, align 8
  %_1 = load i64, ptr %0, align 8
  store i64 %_1, ptr %1, align 8
  %2 = load i64, ptr %1, align 8
  ret i64 %2
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hfeebf8456cd7f980E(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #1 {
start:
  %_29 = alloca { ptr, i64 }, align 8
  %_23 = alloca [1 x { ptr, i64 }], align 8
  %_19 = alloca %"core::fmt::Arguments<'_>", align 8
  %_4 = alloca i8, align 1
; call core::slice::<impl [T]>::len
  %_6 = call i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hc474a61d23f34830E"(ptr align 8 %pieces.0, i64 %pieces.1)
; call core::slice::<impl [T]>::len
  %_8 = call i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17h00c68f98bff8114aE"(ptr align 8 %args.0, i64 %args.1)
  %_5 = icmp ult i64 %_6, %_8
  br i1 %_5, label %bb1, label %bb2

bb2:                                              ; preds = %start
; call core::slice::<impl [T]>::len
  %_11 = call i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hc474a61d23f34830E"(ptr align 8 %pieces.0, i64 %pieces.1)
; call core::slice::<impl [T]>::len
  %_14 = call i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17h00c68f98bff8114aE"(ptr align 8 %args.0, i64 %args.1)
  %1 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %_14, i64 1)
  %_16.0 = extractvalue { i64, i1 } %1, 0
  %_16.1 = extractvalue { i64, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_16.1, i1 false)
  br i1 %2, label %panic, label %bb8

bb1:                                              ; preds = %start
  store i8 1, ptr %_4, align 1
  br label %bb3

bb3:                                              ; preds = %bb8, %bb1
  %3 = load i8, ptr %_4, align 1, !range !1, !noundef !2
  %4 = trunc i8 %3 to i1
  br i1 %4, label %bb9, label %bb11

bb8:                                              ; preds = %bb2
  %_10 = icmp ugt i64 %_11, %_16.0
  %5 = zext i1 %_10 to i8
  store i8 %5, ptr %_4, align 1
  br label %bb3

panic:                                            ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hc253b955bdc90f9aE(ptr align 1 @str.0, i64 28, ptr align 8 @alloc12) #7
  unreachable

bb11:                                             ; preds = %bb3
  store ptr null, ptr %_29, align 8
  %6 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %7 = getelementptr inbounds { ptr, i64 }, ptr %6, i32 0, i32 0
  store ptr %pieces.0, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %6, i32 0, i32 1
  store i64 %pieces.1, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_29, i32 0, i32 0
  %10 = load ptr, ptr %9, align 8, !align !3
  %11 = getelementptr inbounds { ptr, i64 }, ptr %_29, i32 0, i32 1
  %12 = load i64, ptr %11, align 8
  %13 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %10, ptr %13, align 8
  %14 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %12, ptr %14, align 8
  %15 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %16 = getelementptr inbounds { ptr, i64 }, ptr %15, i32 0, i32 0
  store ptr %args.0, ptr %16, align 8
  %17 = getelementptr inbounds { ptr, i64 }, ptr %15, i32 0, i32 1
  store i64 %args.1, ptr %17, align 8
  ret void

bb9:                                              ; preds = %bb3
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hfeebf8456cd7f980E(ptr sret(%"core::fmt::Arguments<'_>") %_19, ptr align 8 @alloc6, i64 1, ptr align 8 @alloc3, i64 0)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h4bcc156bb4faeea2E(ptr %_19, ptr align 8 @alloc14) #7
  unreachable
}

; core::ptr::drop_in_place<core::panic::panic_info::PanicInfo::internal_constructor::NoPayload>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr88drop_in_place$LT$core..panic..panic_info..PanicInfo..internal_constructor..NoPayload$GT$17h8bbeef3baa2dccfaE"(ptr %_1) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::metadata::metadata
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3ptr8metadata8metadata17he10c0507aef4d784E(ptr %ptr.0, i64 %ptr.1) unnamed_addr #1 {
start:
  %_2 = alloca %"core::ptr::metadata::PtrRepr<[&str]>", align 8
  %0 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  store ptr %ptr.0, ptr %0, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  store i64 %ptr.1, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  %3 = load i64, ptr %2, align 8
  ret i64 %3
}

; core::ptr::metadata::metadata
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3ptr8metadata8metadata17hfe001c9cfedb5509E(ptr %ptr.0, i64 %ptr.1) unnamed_addr #1 {
start:
  %_2 = alloca %"core::ptr::metadata::PtrRepr<[core::fmt::ArgumentV1<'_>]>", align 8
  %0 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  store ptr %ptr.0, ptr %0, align 8
  %1 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  store i64 %ptr.1, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  %3 = load i64, ptr %2, align 8
  ret i64 %3
}

; core::panic::panic_info::PanicInfo::internal_constructor
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core5panic10panic_info9PanicInfo20internal_constructor17h4fca2b81056914e2E(ptr sret(%"core::panic::panic_info::PanicInfo<'_>") %0, ptr align 8 %message, ptr align 8 %location, i1 zeroext %can_unwind) unnamed_addr #1 {
start:
  %1 = getelementptr inbounds { ptr, ptr }, ptr %0, i32 0, i32 0
  store ptr @alloc3, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, ptr }, ptr %0, i32 0, i32 1
  store ptr @vtable.2, ptr %2, align 8
  %3 = getelementptr inbounds %"core::panic::panic_info::PanicInfo<'_>", ptr %0, i32 0, i32 1
  store ptr %message, ptr %3, align 8
  %4 = getelementptr inbounds %"core::panic::panic_info::PanicInfo<'_>", ptr %0, i32 0, i32 2
  store ptr %location, ptr %4, align 8
  %5 = getelementptr inbounds %"core::panic::panic_info::PanicInfo<'_>", ptr %0, i32 0, i32 3
  %6 = zext i1 %can_unwind to i8
  store i8 %6, ptr %5, align 8
  ret void
}

; core::panic::location::Location::caller
; Function Attrs: inlinehint uwtable
define internal align 8 ptr @_ZN4core5panic8location8Location6caller17h860ba6a8ab219745E(ptr align 8 %0) unnamed_addr #1 {
start:
  %1 = alloca ptr, align 8
  store ptr %0, ptr %1, align 8
  %2 = load ptr, ptr %1, align 8, !nonnull !2, !align !3, !noundef !2
  ret ptr %2
}

; core::slice::<impl [T]>::len
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17h00c68f98bff8114aE"(ptr align 8 %self.0, i64 %self.1) unnamed_addr #1 {
start:
; call core::ptr::metadata::metadata
  %0 = call i64 @_ZN4core3ptr8metadata8metadata17hfe001c9cfedb5509E(ptr %self.0, i64 %self.1)
  ret i64 %0
}

; core::slice::<impl [T]>::len
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hc474a61d23f34830E"(ptr align 8 %self.0, i64 %self.1) unnamed_addr #1 {
start:
; call core::ptr::metadata::metadata
  %0 = call i64 @_ZN4core3ptr8metadata8metadata17he10c0507aef4d784E(ptr %self.0, i64 %self.1)
  ret i64 %0
}

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
define internal void @_ZN4core9panicking5panic17hc253b955bdc90f9aE(ptr align 1 %expr.0, i64 %expr.1, ptr align 8 %0) unnamed_addr #2 {
start:
  %_8 = alloca [1 x { ptr, i64 }], align 8
  %_4 = alloca %"core::fmt::Arguments<'_>", align 8
  %1 = getelementptr inbounds [1 x { ptr, i64 }], ptr %_8, i64 0, i64 0
  %2 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 0
  store ptr %expr.0, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 1
  store i64 %expr.1, ptr %3, align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hfeebf8456cd7f980E(ptr sret(%"core::fmt::Arguments<'_>") %_4, ptr align 8 %_8, i64 1, ptr align 8 @alloc3, i64 0)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h4bcc156bb4faeea2E(ptr %_4, ptr align 8 %0) #7
  unreachable
}

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
define internal void @_ZN4core9panicking9panic_fmt17h4bcc156bb4faeea2E(ptr %fmt, ptr align 8 %0) unnamed_addr #2 {
start:
  %_7 = alloca ptr, align 8
  %pi = alloca %"core::panic::panic_info::PanicInfo<'_>", align 8
  br i1 false, label %bb1, label %bb2

bb2:                                              ; preds = %start
  store ptr %fmt, ptr %_7, align 8
; call core::panic::location::Location::caller
  %_11 = call align 8 ptr @_ZN4core5panic8location8Location6caller17h860ba6a8ab219745E(ptr align 8 %0)
  %1 = load ptr, ptr %_7, align 8, !align !3
; call core::panic::panic_info::PanicInfo::internal_constructor
  call void @_ZN4core5panic10panic_info9PanicInfo20internal_constructor17h4fca2b81056914e2E(ptr sret(%"core::panic::panic_info::PanicInfo<'_>") %pi, ptr align 8 %1, ptr align 8 %_11, i1 zeroext true)
  call void @rust_begin_unwind(ptr align 8 %pi) #7
  unreachable

bb1:                                              ; preds = %start
  call void @llvm.trap()
  unreachable
}

; probe5::probe
; Function Attrs: uwtable
define void @_ZN6probe55probe17h70d86dcd167bbb30E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hec97dea0634ececcE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hc253b955bdc90f9aE(ptr align 1 @str.1, i64 25, ptr align 8 @alloc16) #7
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hec97dea0634ececcE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #3

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #4

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #5

; Function Attrs: noreturn uwtable
declare void @rust_begin_unwind(ptr align 8) unnamed_addr #6

attributes #0 = { uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { inlinehint uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #3 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #4 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #5 = { cold noreturn nounwind }
attributes #6 = { noreturn uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #7 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i8 0, i8 2}
!2 = !{}
!3 = !{i64 8}
