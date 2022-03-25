macro_rules! count {
    () => {
        0
    };
    (* $($rest:tt)*) => {
        1 + count!($($rest)*)
    };
}

macro_rules! many_ifs {
    ($num:ident, $int:ty, * $($rest:tt)*) => {
        {
            many_ifs!($num, $int, INNER * $($rest)*);
            const COUNT: usize = count!(* $($rest)*);
            COUNT
        }
    };

    ($num:ident, $int:ty, INNER * ) => {};
    ($num:ident, $int:ty, INNER * $($rest:tt)*) => {
        many_ifs!($num, $int, INNER $($rest)*);
        {
            const COUNT: usize = count!($($rest)*);
            const MAX: $int = {
                let mut max: $int = 1;
                let mut i = 0;
                while i < COUNT {
                    max = match max.checked_shl(8) {
                        Some(max) => max,
                        None => 0
                    };
                    i += 1;
                }
                max.wrapping_sub(1)
            };
            if $num <= MAX {
                return COUNT;
            }
        }
    };
}

macro_rules! uint_impl {
    ($export:ident($int:ident): $($rest:tt)*) => {
        pub mod $int {
            #[inline(always)]
            pub const fn loop_pack(mut num: $int) -> usize {
                let mut size = 0;

                loop {
                    num = num >> 8;
                    size += 1;

                    if num == 0 {
                        break size;
                    }
                }
            }

            #[inline(always)]
            pub const fn ifs_pack(
                num: $int
            ) -> usize {
                many_ifs!(num, $int, $($rest)*)
            }
        }

        // ifs_pack performs better in the benchmarks, so we'll reexport that one
        pub use crate::$int::ifs_pack as $export;
    };
}

uint_impl!(u16_packed(u16): **);
uint_impl!(u32_packed(u32): ****);
uint_impl!(u64_packed(u64): ********);
uint_impl!(u128_packed(u128): ****************);
