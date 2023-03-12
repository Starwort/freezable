use std::cell::{Cell, RefCell};

use crate::{Freezable, Frozen, Unfreezable};

impl<T: Freezable> Freezable for RefCell<T> {
    type Frozen = T::Frozen;

    fn freeze(self) -> Frozen<Self> {
        Frozen(self.into_inner().freeze().0)
    }
}
impl<T: Freezable, U: Unfreezable<T>> Unfreezable<T> for RefCell<U> {
    fn thaw(wrapped: <T as Freezable>::Frozen) -> Self {
        RefCell::new(U::thaw(wrapped))
    }
}

impl<T: Freezable> Freezable for Cell<T> {
    type Frozen = T::Frozen;

    fn freeze(self) -> Frozen<Self> {
        Frozen(self.into_inner().freeze().0)
    }
}
impl<T: Freezable, U: Unfreezable<T>> Unfreezable<T> for Cell<U> {
    fn thaw(wrapped: <T as Freezable>::Frozen) -> Self {
        Cell::new(U::thaw(wrapped))
    }
}
