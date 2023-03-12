use core::time::Duration;

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
