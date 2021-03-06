//! `x86` and `x86_64` intrinsics.

use prelude::v1::*;
use mem;

#[macro_use]
mod macros;

macro_rules! types {
    ($(
        $(#[$doc:meta])*
        pub struct $name:ident($($fields:tt)*);
    )*) => ($(
        $(#[$doc])*
        #[derive(Copy, Debug)]
        #[allow(non_camel_case_types)]
        #[repr(simd)]
        pub struct $name($($fields)*);

        #[cfg_attr(feature = "cargo-clippy", allow(expl_impl_clone_on_copy))]
        impl Clone for $name {
            #[inline] // currently needed for correctness
            fn clone(&self) -> $name {
                *self
            }
        }
    )*)
}

types! {
    /// 64-bit wide integer vector type, x86-specific
    ///
    /// This type is the same as the `__m64` type defined by Intel,
    /// representing a 64-bit SIMD register. Usage of this type typically
    /// corresponds to the `mmx` target feature.
    ///
    /// Internally this type may be viewed as:
    ///
    /// * `i8x8` - eight `i8` variables packed together
    /// * `i16x4` - four `i16` variables packed together
    /// * `i32x2` - two `i32` variables packed together
    ///
    /// (as well as unsgined versions). Each intrinsic may interpret the
    /// internal bits differently, check the documentation of the intrinsic
    /// to see how it's being used.
    ///
    /// Note that this means that an instance of `__m64` typically just means
    /// a "bag of bits" which is left up to interpretation at the point of use.
    ///
    /// Most intrinsics using `__m64` are prefixed with `_mm_` and the
    /// integer types tend to correspond to suffixes like "pi8" or "pi32" (not
    /// to be confused with "epiXX", used for `__m128i`).
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(cfg_target_feature, target_feature, stdsimd)]
    /// # #![cfg_attr(not(dox), no_std)]
    /// # #[cfg(not(dox))]
    /// # extern crate std as real_std;
    /// # #[cfg(not(dox))]
    /// # #[macro_use]
    /// # extern crate stdsimd as std;
    /// #[cfg(target_arch = "x86")]
    /// use std::arch::x86::*;
    /// #[cfg(target_arch = "x86_64")]
    /// use std::arch::x86_64::*;
    ///
    /// # fn main() {
    /// # #[target_feature(enable = "mmx")]
    /// # unsafe fn foo() {
    /// let all_bytes_zero = _mm_setzero_si64();
    /// let all_bytes_one = _mm_set1_pi8(1);
    /// let two_i32 = _mm_set_pi32(1, 2);
    /// # }
    /// # if is_x86_feature_detected!("mmx") { unsafe { foo() } }
    /// # }
    /// ```
    pub struct __m64(i64);

    /// 128-bit wide integer vector type, x86-specific
    ///
    /// This type is the same as the `__m128i` type defined by Intel,
    /// representing a 128-bit SIMD register. Usage of this type typically
    /// corresponds to the `sse` and up target features for x86/x86_64.
    ///
    /// Internally this type may be viewed as:
    ///
    /// * `i8x16` - sixteen `i8` variables packed together
    /// * `i16x8` - eight `i16` variables packed together
    /// * `i32x4` - four `i32` variables packed together
    /// * `i64x2` - two `i64` variables packed together
    ///
    /// (as well as unsgined versions). Each intrinsic may interpret the
    /// internal bits differently, check the documentation of the intrinsic
    /// to see how it's being used.
    ///
    /// Note that this means that an instance of `__m128i` typically just means
    /// a "bag of bits" which is left up to interpretation at the point of use.
    ///
    /// Most intrinsics using `__m128i` are prefixed with `_mm_` and the
    /// integer types tend to correspond to suffixes like "epi8" or "epi32".
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(cfg_target_feature, target_feature, stdsimd)]
    /// # #![cfg_attr(not(dox), no_std)]
    /// # #[cfg(not(dox))]
    /// # extern crate std as real_std;
    /// # #[cfg(not(dox))]
    /// # #[macro_use]
    /// # extern crate stdsimd as std;
    /// #[cfg(target_arch = "x86")]
    /// use std::arch::x86::*;
    /// #[cfg(target_arch = "x86_64")]
    /// use std::arch::x86_64::*;
    ///
    /// # fn main() {
    /// # #[target_feature(enable = "sse2")]
    /// # unsafe fn foo() {
    /// let all_bytes_zero = _mm_setzero_si128();
    /// let all_bytes_one = _mm_set1_epi8(1);
    /// let four_i32 = _mm_set_epi32(1, 2, 3, 4);
    /// # }
    /// # if is_x86_feature_detected!("sse2") { unsafe { foo() } }
    /// # }
    /// ```
    pub struct __m128i(i64, i64);

    /// 128-bit wide set of four `f32` types, x86-specific
    ///
    /// This type is the same as the `__m128` type defined by Intel,
    /// representing a 128-bit SIMD register which internally is consisted of
    /// four packed `f32` instances. Usage of this type typically corresponds
    /// to the `sse` and up target features for x86/x86_64.
    ///
    /// Note that unlike `__m128i`, the integer version of the 128-bit
    /// registers, this `__m128` type has *one* interpretation. Each instance
    /// of `__m128` always corresponds to `f32x4`, or four `f32` types packed
    /// together.
    ///
    /// Most intrinsics using `__m128` are prefixed with `_mm_` and are
    /// suffixed with "ps" (or otherwise contain "ps"). Not to be confused with
    /// "pd" which is used for `__m128d`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(cfg_target_feature, target_feature, stdsimd)]
    /// # #![cfg_attr(not(dox), no_std)]
    /// # #[cfg(not(dox))]
    /// # extern crate std as real_std;
    /// # #[cfg(not(dox))]
    /// # #[macro_use]
    /// # extern crate stdsimd as std;
    /// #[cfg(target_arch = "x86")]
    /// use std::arch::x86::*;
    /// #[cfg(target_arch = "x86_64")]
    /// use std::arch::x86_64::*;
    ///
    /// # fn main() {
    /// # #[target_feature(enable = "sse")]
    /// # unsafe fn foo() {
    /// let four_zeros = _mm_setzero_ps();
    /// let four_ones = _mm_set1_ps(1.0);
    /// let four_floats = _mm_set_ps(1.0, 2.0, 3.0, 4.0);
    /// # }
    /// # if is_x86_feature_detected!("sse") { unsafe { foo() } }
    /// # }
    /// ```
    pub struct __m128(f32, f32, f32, f32);

    /// 128-bit wide set of two `f64` types, x86-specific
    ///
    /// This type is the same as the `__m128d` type defined by Intel,
    /// representing a 128-bit SIMD register which internally is consisted of
    /// two packed `f64` instances. Usage of this type typically corresponds
    /// to the `sse` and up target features for x86/x86_64.
    ///
    /// Note that unlike `__m128i`, the integer version of the 128-bit
    /// registers, this `__m128d` type has *one* interpretation. Each instance
    /// of `__m128d` always corresponds to `f64x2`, or two `f64` types packed
    /// together.
    ///
    /// Most intrinsics using `__m128d` are prefixed with `_mm_` and are
    /// suffixed with "pd" (or otherwise contain "pd"). Not to be confused with
    /// "ps" which is used for `__m128`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(cfg_target_feature, target_feature, stdsimd)]
    /// # #![cfg_attr(not(dox), no_std)]
    /// # #[cfg(not(dox))]
    /// # extern crate std as real_std;
    /// # #[cfg(not(dox))]
    /// # #[macro_use]
    /// # extern crate stdsimd as std;
    /// #[cfg(target_arch = "x86")]
    /// use std::arch::x86::*;
    /// #[cfg(target_arch = "x86_64")]
    /// use std::arch::x86_64::*;
    ///
    /// # fn main() {
    /// # #[target_feature(enable = "sse")]
    /// # unsafe fn foo() {
    /// let two_zeros = _mm_setzero_pd();
    /// let two_ones = _mm_set1_pd(1.0);
    /// let two_floats = _mm_set_pd(1.0, 2.0);
    /// # }
    /// # if is_x86_feature_detected!("sse") { unsafe { foo() } }
    /// # }
    /// ```
    pub struct __m128d(f64, f64);

    /// 256-bit wide integer vector type, x86-specific
    ///
    /// This type is the same as the `__m256i` type defined by Intel,
    /// representing a 256-bit SIMD register. Usage of this type typically
    /// corresponds to the `avx` and up target features for x86/x86_64.
    ///
    /// Internally this type may be viewed as:
    ///
    /// * `i8x32` - thirty two `i8` variables packed together
    /// * `i16x16` - sixteen `i16` variables packed together
    /// * `i32x8` - eight `i32` variables packed together
    /// * `i64x4` - four `i64` variables packed together
    ///
    /// (as well as unsgined versions). Each intrinsic may interpret the
    /// internal bits differently, check the documentation of the intrinsic
    /// to see how it's being used.
    ///
    /// Note that this means that an instance of `__m256i` typically just means
    /// a "bag of bits" which is left up to interpretation at the point of use.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(cfg_target_feature, target_feature, stdsimd)]
    /// # #![cfg_attr(not(dox), no_std)]
    /// # #[cfg(not(dox))]
    /// # extern crate std as real_std;
    /// # #[cfg(not(dox))]
    /// # #[macro_use]
    /// # extern crate stdsimd as std;
    /// #[cfg(target_arch = "x86")]
    /// use std::arch::x86::*;
    /// #[cfg(target_arch = "x86_64")]
    /// use std::arch::x86_64::*;
    ///
    /// # fn main() {
    /// # #[target_feature(enable = "avx")]
    /// # unsafe fn foo() {
    /// let all_bytes_zero = _mm256_setzero_si256();
    /// let all_bytes_one = _mm256_set1_epi8(1);
    /// let eight_i32 = _mm256_set_epi32(1, 2, 3, 4, 5, 6, 7, 8);
    /// # }
    /// # if is_x86_feature_detected!("avx") { unsafe { foo() } }
    /// # }
    /// ```
    pub struct __m256i(i64, i64, i64, i64);

    /// 256-bit wide set of eight `f32` types, x86-specific
    ///
    /// This type is the same as the `__m256` type defined by Intel,
    /// representing a 256-bit SIMD register which internally is consisted of
    /// eight packed `f32` instances. Usage of this type typically corresponds
    /// to the `avx` and up target features for x86/x86_64.
    ///
    /// Note that unlike `__m256i`, the integer version of the 256-bit
    /// registers, this `__m256` type has *one* interpretation. Each instance
    /// of `__m256` always corresponds to `f32x8`, or eight `f32` types packed
    /// together.
    ///
    /// Most intrinsics using `__m256` are prefixed with `_mm256_` and are
    /// suffixed with "ps" (or otherwise contain "ps"). Not to be confused with
    /// "pd" which is used for `__m256d`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(cfg_target_feature, target_feature, stdsimd)]
    /// # #![cfg_attr(not(dox), no_std)]
    /// # #[cfg(not(dox))]
    /// # extern crate std as real_std;
    /// # #[cfg(not(dox))]
    /// # #[macro_use]
    /// # extern crate stdsimd as std;
    /// #[cfg(target_arch = "x86")]
    /// use std::arch::x86::*;
    /// #[cfg(target_arch = "x86_64")]
    /// use std::arch::x86_64::*;
    ///
    /// # fn main() {
    /// # #[target_feature(enable = "sse")]
    /// # unsafe fn foo() {
    /// let eight_zeros = _mm256_setzero_ps();
    /// let eight_ones = _mm256_set1_ps(1.0);
    /// let eight_floats = _mm256_set_ps(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    /// # }
    /// # if is_x86_feature_detected!("sse") { unsafe { foo() } }
    /// # }
    /// ```
    pub struct __m256(f32, f32, f32, f32, f32, f32, f32, f32);

    /// 256-bit wide set of four `f64` types, x86-specific
    ///
    /// This type is the same as the `__m256d` type defined by Intel,
    /// representing a 256-bit SIMD register which internally is consisted of
    /// four packed `f64` instances. Usage of this type typically corresponds
    /// to the `avx` and up target features for x86/x86_64.
    ///
    /// Note that unlike `__m256i`, the integer version of the 256-bit
    /// registers, this `__m256d` type has *one* interpretation. Each instance
    /// of `__m256d` always corresponds to `f64x4`, or four `f64` types packed
    /// together.
    ///
    /// Most intrinsics using `__m256d` are prefixed with `_mm256_` and are
    /// suffixed with "pd" (or otherwise contain "pd"). Not to be confused with
    /// "ps" which is used for `__m256`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(cfg_target_feature, target_feature, stdsimd)]
    /// # #![cfg_attr(not(dox), no_std)]
    /// # #[cfg(not(dox))]
    /// # extern crate std as real_std;
    /// # #[cfg(not(dox))]
    /// # #[macro_use]
    /// # extern crate stdsimd as std;
    /// #[cfg(target_arch = "x86")]
    /// use std::arch::x86::*;
    /// #[cfg(target_arch = "x86_64")]
    /// use std::arch::x86_64::*;
    ///
    /// # fn main() {
    /// # #[target_feature(enable = "avx")]
    /// # unsafe fn foo() {
    /// let four_zeros = _mm256_setzero_pd();
    /// let four_ones = _mm256_set1_pd(1.0);
    /// let four_floats = _mm256_set_pd(1.0, 2.0, 3.0, 4.0);
    /// # }
    /// # if is_x86_feature_detected!("avx") { unsafe { foo() } }
    /// # }
    /// ```
    pub struct __m256d(f64, f64, f64, f64);
}

#[cfg(test)]
mod test;
#[cfg(test)]
pub use self::test::*;

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub(crate) trait m128iExt: Sized {
    fn as_m128i(self) -> __m128i;

    #[inline]
    fn as_u8x16(self) -> ::coresimd::simd::u8x16 {
        unsafe { mem::transmute(self.as_m128i()) }
    }

    #[inline]
    fn as_u16x8(self) -> ::coresimd::simd::u16x8 {
        unsafe { mem::transmute(self.as_m128i()) }
    }

    #[inline]
    fn as_u32x4(self) -> ::coresimd::simd::u32x4 {
        unsafe { mem::transmute(self.as_m128i()) }
    }

    #[inline]
    fn as_u64x2(self) -> ::coresimd::simd::u64x2 {
        unsafe { mem::transmute(self.as_m128i()) }
    }

    #[inline]
    fn as_i8x16(self) -> ::coresimd::simd::i8x16 {
        unsafe { mem::transmute(self.as_m128i()) }
    }

    #[inline]
    fn as_i16x8(self) -> ::coresimd::simd::i16x8 {
        unsafe { mem::transmute(self.as_m128i()) }
    }

    #[inline]
    fn as_i32x4(self) -> ::coresimd::simd::i32x4 {
        unsafe { mem::transmute(self.as_m128i()) }
    }

    #[inline]
    fn as_i64x2(self) -> ::coresimd::simd::i64x2 {
        unsafe { mem::transmute(self.as_m128i()) }
    }
}

impl m128iExt for __m128i {
    #[inline]
    fn as_m128i(self) -> Self {
        self
    }
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub(crate) trait m256iExt: Sized {
    fn as_m256i(self) -> __m256i;

    #[inline]
    fn as_u8x32(self) -> ::coresimd::simd::u8x32 {
        unsafe { mem::transmute(self.as_m256i()) }
    }

    #[inline]
    fn as_u16x16(self) -> ::coresimd::simd::u16x16 {
        unsafe { mem::transmute(self.as_m256i()) }
    }

    #[inline]
    fn as_u32x8(self) -> ::coresimd::simd::u32x8 {
        unsafe { mem::transmute(self.as_m256i()) }
    }

    #[inline]
    fn as_u64x4(self) -> ::coresimd::simd::u64x4 {
        unsafe { mem::transmute(self.as_m256i()) }
    }

    #[inline]
    fn as_i8x32(self) -> ::coresimd::simd::i8x32 {
        unsafe { mem::transmute(self.as_m256i()) }
    }

    #[inline]
    fn as_i16x16(self) -> ::coresimd::simd::i16x16 {
        unsafe { mem::transmute(self.as_m256i()) }
    }

    #[inline]
    fn as_i32x8(self) -> ::coresimd::simd::i32x8 {
        unsafe { mem::transmute(self.as_m256i()) }
    }

    #[inline]
    fn as_i64x4(self) -> ::coresimd::simd::i64x4 {
        unsafe { mem::transmute(self.as_m256i()) }
    }
}

impl m256iExt for __m256i {
    #[inline]
    fn as_m256i(self) -> Self {
        self
    }
}

use coresimd::simd::{b8x16, b8x32, b8x8, f32x4, f32x8, f64x2, f64x4, i16x16,
                     i16x4, i16x8, i32x2, i32x4, i32x8, i64x2, i64x4, i8x16,
                     i8x32, i8x8, u16x16, u16x4, u16x8, u32x2, u32x4, u32x8,
                     u64x2, u64x4, u8x16, u8x32, u8x8};

impl_from_bits_!(__m64: u32x2, i32x2, u16x4, i16x4, u8x8, i8x8, b8x8);
impl_from_bits_!(
    __m128: u64x2,
    i64x2,
    f64x2,
    u32x4,
    i32x4,
    f32x4,
    u16x8,
    i16x8,
    u8x16,
    i8x16,
    b8x16
);
impl_from_bits_!(
    __m128i: u64x2,
    i64x2,
    f64x2,
    u32x4,
    i32x4,
    f32x4,
    u16x8,
    i16x8,
    u8x16,
    i8x16,
    b8x16
);
impl_from_bits_!(
    __m128d: u64x2,
    i64x2,
    f64x2,
    u32x4,
    i32x4,
    f32x4,
    u16x8,
    i16x8,
    u8x16,
    i8x16,
    b8x16
);
impl_from_bits_!(
    __m256: u64x4,
    i64x4,
    f64x4,
    u32x8,
    i32x8,
    f32x8,
    u16x16,
    i16x16,
    u8x32,
    i8x32,
    b8x32
);
impl_from_bits_!(
    __m256i: u64x4,
    i64x4,
    f64x4,
    u32x8,
    i32x8,
    f32x8,
    u16x16,
    i16x16,
    u8x32,
    i8x32,
    b8x32
);
impl_from_bits_!(
    __m256d: u64x4,
    i64x4,
    f64x4,
    u32x8,
    i32x8,
    f32x8,
    u16x16,
    i16x16,
    u8x32,
    i8x32,
    b8x32
);

mod eflags;
pub use self::eflags::*;

#[cfg(dont_compile_me)] // TODO: need to upstream `fxsr` target feature
mod fxsr;
#[cfg(dont_compile_me)] // TODO: need to upstream `fxsr` target feature
pub use self::fxsr::*;

mod bswap;
pub use self::bswap::*;

mod rdtsc;
pub use self::rdtsc::*;

mod cpuid;
pub use self::cpuid::*;
mod xsave;
pub use self::xsave::*;

mod sse;
pub use self::sse::*;
mod sse2;
pub use self::sse2::*;
mod sse3;
pub use self::sse3::*;
mod ssse3;
pub use self::ssse3::*;
mod sse41;
pub use self::sse41::*;
mod sse42;
pub use self::sse42::*;
mod avx;
pub use self::avx::*;
mod avx2;
pub use self::avx2::*;

mod abm;
pub use self::abm::*;
mod bmi;
pub use self::bmi::*;

mod bmi2;
pub use self::bmi2::*;

#[cfg(not(feature = "intel_sde"))]
mod sse4a;
#[cfg(not(feature = "intel_sde"))]
pub use self::sse4a::*;

#[cfg(not(feature = "intel_sde"))]
mod tbm;
#[cfg(not(feature = "intel_sde"))]
pub use self::tbm::*;

mod mmx;
pub use self::mmx::*;

mod pclmulqdq;
pub use self::pclmulqdq::*;

mod aes;
pub use self::aes::*;

mod rdrand;
pub use self::rdrand::*;
