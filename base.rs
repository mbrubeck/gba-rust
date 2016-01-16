pub mod option {
    pub enum Option<T> {
        Some(T),
        None
    }
}

pub mod kinds {
    #[lang="sized"]
    pub trait Sized {}
}

pub mod fail {
    pub fn abort() -> ! { loop {} }

    #[inline]
    #[lang="fail_bounds_check"]
    pub fn fail_bounds_check(_: *const u8, _: usize, _: usize, _: usize) -> ! {
        abort()
    }

    #[inline]
    #[lang="fail_"]
    pub fn fail_(_: *const u8, _: *const u8, _: usize) -> ! {
        abort()
    }
}

pub mod num {
    pub trait Num {
        fn cmp(&self, o: &Self) -> isize;
        fn add(&self, o: &Self) -> Self;
        fn one() -> Self;
    }

    macro_rules! num_impl(
        ($tpe: ty) => {
            impl Num for $tpe {
                fn cmp(&self, o: &$tpe) -> isize {
                    if *self < *o { -1 }
                    else if *self > *o { 1 }
                    else { 0 }
                }
                fn add(&self, o: &$tpe) -> $tpe { *self + *o }
                fn one() -> $tpe { 1 }
            }
        };
    );
    num_impl!(isize);
    num_impl!(usize);
    num_impl!(u32);
}

pub mod clone {
    pub trait Clone {
        fn clone(&self) -> Self;
    }

    macro_rules! clone_impl(
        ($tpe: ty) => {
            impl Clone for $tpe {
                fn clone(&self) -> $tpe { *self }
            }
        };
    );

    clone_impl!(isize);
    clone_impl!(usize);
    clone_impl!(u32);
}

pub mod iter {
    use super::option::*;
    use super::num::*;
    use super::clone::*;
    
    pub trait Iterator<A> {
        fn next(&mut self) -> Option<A>;
    }
    
    pub struct Range<T> {
        state: T,
        stop: T
    }

    pub fn range<T: Num>(start: T, stop: T) -> Range<T> { Range { state: start, stop: stop } }

    impl<T: Num + Clone> Iterator<T> for Range<T> {
        fn next(&mut self) -> Option<T> {
            if self.state.cmp(&self.stop) < 0 {
                let result = Some(self.state.clone());
                self.state = self.state.add(&Num::one());
                result
            } else {
                None
            }
        }
    }
}

#[allow(dead_code)]
pub mod rand {
    pub use super::iter::*;
    pub use super::option::*;
    pub struct Rand { state: u32 }

    impl Rand {
        pub fn new(seed: u32) -> Rand { Rand { state: seed } }
        pub fn next_bool(&mut self) -> bool {
            self.state = self.state * 1664525u32 + 1013904223u32;
            self.state & 0x80000000u32 != 0
        }
        pub fn next_u8(&mut self) -> u8 {
            let mut result = 0u8;
            for i in 0..8 {
                result |= (self.next_bool() as u8) << i;
            }
            result
        }
    }
}

pub mod prelude {
    pub use super::option::{ Option, Some, None };
    pub use super::kinds::*;
    pub use super::fail::*;
    pub use super::num::*;
    pub use super::clone::*;
    pub use super::iter::{ Iterator, Range, range };
    pub use super::rand::Rand;
}
