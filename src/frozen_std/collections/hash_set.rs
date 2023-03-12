use std::collections::hash_map::{DefaultHasher, RandomState};
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::Deref;

use crate::{Freezable, Frozen, Unfreezable};

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FrozenSet<T: Hash + Eq, S: BuildHasher = RandomState>(HashSet<T, S>);
impl<T: Freezable, RK: Hash + Eq + Unfreezable<T>, S: BuildHasher + Default>
    Unfreezable<HashSet<T, S>> for HashSet<RK, S>
where
    T::Frozen: Hash + Eq,
{
    fn thaw(FrozenSet(val): <HashSet<T, S> as Freezable>::Frozen) -> HashSet<RK, S> {
        val.into_iter().map(Frozen::thaw).collect()
    }
}
impl<K: Hash + Eq, S: BuildHasher> Deref for FrozenSet<K, S> {
    type Target = HashSet<K, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Hash + Eq, S: BuildHasher> PartialEq for FrozenSet<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: Hash + Eq, S: BuildHasher> Eq for FrozenSet<T, S> {
}
#[allow(clippy::zero_sized_map_values)]
impl<T: Freezable, RK: Hash + Eq + Unfreezable<T>, S: BuildHasher + Default>
    Unfreezable<HashSet<T, S>> for HashMap<RK, (), S>
where
    T::Frozen: Hash + Eq,
{
    fn thaw(
        FrozenSet(val): <HashSet<T, S> as Freezable>::Frozen,
    ) -> HashMap<RK, (), S> {
        val.into_iter().map(|val| (val.thaw(), ())).collect()
    }
}

/// Freeze a [`HashSet`].
///
/// For `T` where freezing is a no-op, this is a no-op.
impl<T: Freezable, S: BuildHasher + Default> Freezable for HashSet<T, S>
where
    T::Frozen: Hash + Eq,
{
    type Frozen = FrozenSet<Frozen<T>, S>;

    fn freeze(self) -> Frozen<Self> {
        Frozen(FrozenSet(
            self.into_iter()
                .map(Freezable::freeze)
                .collect::<HashSet<Frozen<T>, S>>(),
        ))
    }
}

impl<T: Hash + Eq, S: BuildHasher> Hash for FrozenSet<T, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash = 0;
        for key in &self.0 {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hash ^= hasher.finish();
        }
        state.write_u64(hash);
    }
}
