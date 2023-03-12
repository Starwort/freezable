use crate::{Freezable, Frozen, Unfreezable};

impl Freezable for String {
    type Frozen = Self;

    fn freeze(self) -> Frozen<Self> {
        Frozen(self)
    }
}

impl Unfreezable<String> for String {
    fn thaw(wrapped: <String as Freezable>::Frozen) -> Self {
        wrapped
    }
}
