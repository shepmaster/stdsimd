//! `coresimd`

#[macro_use]
mod ppsv;

/// Platform independent SIMD vector types and operations.
///
/// This is an **unstable** module for portable SIMD operations. This module
/// has not yet gone through an RFC and is likely to change, but feedback is
/// always welcome!
#[unstable(feature = "stdsimd", issue = "0")]
pub mod simd {
    pub use coresimd::ppsv::*;
}

/// Platform dependent vendor intrinsics.
///
/// This documentation is for the version of this module in the `coresimd`
/// crate, but you probably want to use the [`stdsimd` crate][stdsimd] which
/// should have more complete documentation.
///
/// [stdsimd]: https://rust-lang-nursery.github.io/stdsimd/x86_64/stdsimd/arch/index.html
///
/// Also note that while this module may appear to contains the intrinsics for
/// only one platform it actually contains intrinsics for multiple platforms
/// compiled in conditionally. For other platforms of stdsimd see:
///
/// * [`x86`]
/// * [`x86_64`]
/// * [`arm`]
/// * [`aarch64`]
///
/// [`x86`]: https://rust-lang-nursery.github.io/stdsimd/x86/stdsimd/arch/index.html
/// [`x86_64`]: https://rust-lang-nursery.github.io/stdsimd/x86_64/stdsimd/arch/index.html
/// [`arm`]: https://rust-lang-nursery.github.io/stdsimd/arm/stdsimd/arch/index.html
/// [`aarch64`]: https://rust-lang-nursery.github.io/stdsimd/aarch64/stdsimd/arch/index.html
#[unstable(feature = "stdsimd", issue = "0")]
pub mod arch {
    /// Platform-specific intrinsics for the `x86` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(target_arch = "x86")]
    pub mod x86 {
        pub use coresimd::x86::*;
    }

    /// Platform-specific intrinsics for the `x86_64` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(target_arch = "x86_64")]
    pub mod x86_64 {
        pub use coresimd::x86::*;
        pub use coresimd::x86_64::*;
    }

    /// Platform-specific intrinsics for the `arm` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(target_arch = "arm")]
    pub mod arm {
        pub use coresimd::arm::*;
    }

    /// Platform-specific intrinsics for the `aarch64` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(target_arch = "aarch64")]
    pub mod aarch64 {
        pub use coresimd::arm::*;
        pub use coresimd::aarch64::*;
    }
}

mod simd_llvm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod arm;
#[cfg(target_arch = "aarch64")]
mod aarch64;

mod nvptx;
