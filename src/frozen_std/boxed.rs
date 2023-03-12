use std::boxed::Box;

use crate::{Freezable, Frozen, Unfreezable};

impl<T: Freezable> Freezable for Box<T> {
    type Frozen = T::Frozen;

    fn freeze(self) -> Frozen<Self> {
        // Nasty hack required to satisfy the type system.
        Frozen((*self).freeze().0)
    }
}

impl<U: Unfreezable<T>, T: Freezable> Unfreezable<T> for Box<U> {
    fn thaw(wrapped: <T as Freezable>::Frozen) -> Self {
        Box::new(U::thaw(wrapped))
    }
}
