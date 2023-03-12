use core::time::Duration;
use core::ops::Deref;

use crate::{impl_self_freezable, Freezable, Frozen, Unfreezable};
macro_rules! tuple_impl {
    ($($param:ident),*) => {
        impl<$($param: Freezable),*> Freezable for ($($param),*) {
            type Frozen = ($(Frozen<$param>),*);

            fn freeze(self) -> Frozen<Self> {
                #[allow(non_snake_case)]
                let ($($param),*) = self;

                Frozen(($($param.freeze()),*))
            }
        }
        #[allow(clippy::unused_unit)]
        impl<$($param: Freezable + Unfreezable<$param>),*> Unfreezable<($($param),*)> for ($($param),*) {
            fn thaw(wrapped: <($($param),*) as Freezable>::Frozen) -> Self {
                #[allow(non_snake_case)]
                let ($($param),*) = wrapped;

                ($($param.thaw()),*)
            }
        }
    };
}
tuple_impl!();
tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);
tuple_impl!(A, B, C, D, E, F, G);
tuple_impl!(A, B, C, D, E, F, G, H);
tuple_impl!(A, B, C, D, E, F, G, H, I);
tuple_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

impl<T: Freezable, const N: usize> Freezable for [T; N] {
    type Frozen = [Frozen<T>; N];

    fn freeze(self) -> Frozen<Self> {
        Frozen(self.map(Freezable::freeze))
    }
}
impl<T: Unfreezable<U>, U: Freezable, const N: usize> Unfreezable<[U; N]> for [T; N] {
    fn thaw(wrapped: <[U; N] as Freezable>::Frozen) -> Self {
        wrapped.map(Frozen::thaw)
    }
}

impl<T: Freezable> Freezable for Option<T> {
    type Frozen = Option<Frozen<T>>;

    fn freeze(self) -> Frozen<Self> {
        Frozen(self.map(Freezable::freeze))
    }
}
impl<T: Unfreezable<U>, U: Freezable> Unfreezable<Option<U>> for Option<T> {
    fn thaw(wrapped: <Option<U> as Freezable>::Frozen) -> Self {
        wrapped.map(Frozen::thaw)
    }
}

impl_self_freezable!(bool);
impl_self_freezable!(char);
impl_self_freezable!(f32);
impl_self_freezable!(f64);
impl_self_freezable!(i8);
impl_self_freezable!(i16);
impl_self_freezable!(i32);
impl_self_freezable!(i64);
impl_self_freezable!(i128);
impl_self_freezable!(isize);
impl_self_freezable!(u8);
impl_self_freezable!(u16);
impl_self_freezable!(u32);
impl_self_freezable!(u64);
impl_self_freezable!(u128);
impl_self_freezable!(usize);
impl_self_freezable!(&'a str, 'a);

impl Freezable for Duration {
    type Frozen = Self;

    fn freeze(self) -> Frozen<Self> {
        Frozen(self)
    }
}
impl Unfreezable<Duration> for Duration {
    fn thaw(wrapped: <Duration as Freezable>::Frozen) -> Self {
        wrapped
    }
}

// Freezable bound is used to enforce that the type does not contain `UnsafeCell`.
impl<T: Freezable> Freezable for &T {
    type Frozen = Self;

    fn freeze(self) -> Frozen<Self> {
        Frozen(self)
    }
}
impl<'a, T: Freezable> Unfreezable<&'a T> for &'a T {
    fn thaw(wrapped: <&'a T as Freezable>::Frozen) -> Self {
        wrapped
    }
}

#[derive(Debug)]
pub struct FrozenMutRef<'a, T: Freezable>(&'a mut T);
impl<'a, T: Freezable> Deref for FrozenMutRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: Freezable> Freezable for &'a mut T {
    type Frozen = FrozenMutRef<'a, T>;

    fn freeze(self) -> Frozen<Self> {
        Frozen(FrozenMutRef(self))
    }
}
impl<'a, T: Freezable> Unfreezable<&'a mut T> for &'a mut T {
    fn thaw(FrozenMutRef(wrapped): <&'a mut T as Freezable>::Frozen) -> Self {
        wrapped
    }
}
impl<'a, T: Freezable> Unfreezable<&'a mut T> for &'a T {
    fn thaw(FrozenMutRef(wrapped): <&'a mut T as Freezable>::Frozen) -> Self {
        wrapped
    }
}

