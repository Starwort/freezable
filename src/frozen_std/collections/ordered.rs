use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

use crate::{Freezable, Frozen, Unfreezable};

macro_rules! freezable_impl {
    ($($params:ident),* => $impl_type:ty, $frozen_type:ty) => {
        impl<$($params: Freezable),*> Freezable for $impl_type {
            type Frozen = Vec<Frozen<$frozen_type>>;

            fn freeze(self) -> Frozen<Self> {
                Frozen(self.into_iter().map(Freezable::freeze).collect())
            }
        }
    };
}
macro_rules! unfreezable_impl {
    (Vec < T > , T) => {
        #[allow(unused_parens)]
        impl<
            U: Unfreezable<T>,
            T: Freezable
        > Unfreezable<Vec<T>> for Vec<U> {
            fn thaw(wrapped: <Vec<T> as Freezable>::Frozen) -> Self {
                wrapped.into_iter().map(Frozen::thaw).collect()
            }
        }
    };
    ($unfreeze_type:ty, $($params:ident),* $(| $($bound_ty:ty : $bounds:tt),*)?) => {
        #[allow(unused_parens)]
        impl<
            U: Unfreezable<($($params),*)>,
            $($params: Freezable),*
        > Unfreezable<$unfreeze_type> for Vec<U> {
            fn thaw(wrapped: <$unfreeze_type as Freezable>::Frozen) -> Self {
                wrapped.into_iter().map(Frozen::thaw).collect()
            }
        }
        #[allow(unused_parens)]
        impl<U: Freezable, $($params),*> Unfreezable<Vec<U>> for $unfreeze_type where
            ($($params),*): Unfreezable<U>,
            $($($bound_ty : $bounds),*)?
        {
            fn thaw(wrapped: <Vec<U> as Freezable>::Frozen) -> Self {
                wrapped.into_iter().map(Frozen::thaw).collect()
            }
        }
    };
}
freezable_impl!(T => Vec<T>, T);
unfreezable_impl!(Vec<T>, T);
freezable_impl!(T => VecDeque<T>, T);
unfreezable_impl!(VecDeque<T>, T);
freezable_impl!(K, V => BTreeMap<K, V>, (K, V));
unfreezable_impl!(BTreeMap<K, V>, K, V | K: Ord, V: Ord);
freezable_impl!(T => BTreeSet<T>, T);
unfreezable_impl!(BTreeSet<T>, T | T: Ord);
freezable_impl!(T => LinkedList<T>, T);
unfreezable_impl!(LinkedList<T>, T);
impl<T: Freezable + Ord> Freezable for BinaryHeap<T> {
    type Frozen = Vec<Frozen<T>>;

    fn freeze(self) -> Frozen<Self> {
        Frozen(
            self.into_sorted_vec()
                .into_iter()
                .map(Freezable::freeze)
                .collect(),
        )
    }
}
impl<T: Freezable + Ord, U: Unfreezable<T>> Unfreezable<BinaryHeap<T>> for Vec<U> {
    fn thaw(wrapped: <BinaryHeap<T> as Freezable>::Frozen) -> Self {
        wrapped.into_iter().map(Frozen::thaw).collect()
    }
}
