//! Lane-wise boolean vector reductions.

macro_rules! impl_bool_reductions {
    ($id:ident) => {
        impl $id {
            /// Are `all` vector lanes `true`?
            #[inline]
            pub fn all(self) -> bool {
                self.and()
            }
            /// Is `any` vector lanes `true`?
            #[inline]
            pub fn any(self) -> bool {
                self.or()
            }
            /// Are `all` vector lanes `false`?
            #[inline]
            pub fn none(self) -> bool {
                !self.or()
            }
        }
    }
}

#[cfg(test)]
macro_rules! test_bool_reductions {
    ($id:ident) => {
        #[test]
        fn all() {
            use ::coresimd::simd::$id;

            let a = $id::splat(true);
            assert!(a.all());
            let a = $id::splat(false);
            assert!(!a.all());

            for i in 0..$id::lanes() {
                let mut a = $id::splat(true);
                a = a.replace(i, false);
                assert!(!a.all());
                let mut a = $id::splat(false);
                a = a.replace(i, true);
                assert!(!a.all());
            }
        }
        #[test]
        fn any() {
            use ::coresimd::simd::$id;

            let a = $id::splat(true);
            assert!(a.any());
            let a = $id::splat(false);
            assert!(!a.any());

            for i in 0..$id::lanes() {
                let mut a = $id::splat(true);
                a = a.replace(i, false);
                assert!(a.any());
                let mut a = $id::splat(false);
                a = a.replace(i, true);
                assert!(a.any());
            }
        }
        #[test]
        fn none() {
            use ::coresimd::simd::$id;

            let a = $id::splat(true);
            assert!(!a.none());
            let a = $id::splat(false);
            assert!(a.none());

            for i in 0..$id::lanes() {
                let mut a = $id::splat(true);
                a = a.replace(i, false);
                assert!(!a.none());
                let mut a = $id::splat(false);
                a = a.replace(i, true);
                assert!(!a.none());
            }
        }
    }
}
