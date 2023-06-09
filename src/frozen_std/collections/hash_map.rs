use core::borrow::Borrow;
use core::ops::Index;
use std::collections::hash_map::{DefaultHasher, IntoIter, Iter, RandomState};
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::Deref;

use crate::{Freezable, Frozen, Unfreezable};

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct FrozenMap<K: Hash + Eq, V, S: BuildHasher = RandomState>(HashMap<K, V, S>);
impl<
        K: Freezable,
        V: Freezable,
        RK: Hash + Eq + Unfreezable<K>,
        RV: Unfreezable<V>,
        S: BuildHasher + Default,
    > Unfreezable<HashMap<K, V, S>> for HashMap<RK, RV, S>
where
    K::Frozen: Hash + Eq,
{
    fn thaw(
        FrozenMap(val): <HashMap<K, V, S> as Freezable>::Frozen,
    ) -> HashMap<RK, RV, S> {
        val.into_iter().map(|(k, v)| (k.thaw(), v.thaw())).collect()
    }
}
impl<K: Hash + Eq, V, S: BuildHasher, B: Freezable> Index<B> for FrozenMap<K, V, S>
where
    K: Borrow<Frozen<B>>,
    B::Frozen: Hash + Eq,
{
    type Output = V;

    fn index(&self, index: B) -> &Self::Output {
        self.0.index(&index.freeze())
    }
}
impl<K: Hash + Eq, V: PartialEq, S: BuildHasher> PartialEq for FrozenMap<K, V, S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: Hash + Eq, V: Eq, S: BuildHasher> Eq for FrozenMap<K, V, S> {
}
impl<K: Hash + Eq, V, S: BuildHasher> IntoIterator for FrozenMap<K, V, S> {
    type IntoIter = IntoIter<K, V>;
    type Item = (K, V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<K: Hash + Eq, V, S: BuildHasher> FrozenMap<K, V, S> {
    pub fn iter(&self) -> Iter<K, V> {
        self.0.iter()
    }
}
impl<K: Hash + Eq, V, S: BuildHasher> Deref for FrozenMap<K, V, S> {
    type Target = HashMap<K, V, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[allow(clippy::zero_sized_map_values)]
impl<K: Freezable, RT: Hash + Eq + Unfreezable<K>, S: BuildHasher + Default>
    Unfreezable<HashMap<K, (), S>> for HashSet<RT, S>
where
    K::Frozen: Hash + Eq,
{
    fn thaw(
        FrozenMap(val): <HashMap<K, (), S> as Freezable>::Frozen,
    ) -> HashSet<RT, S> {
        val.into_keys().map(Frozen::thaw).collect()
    }
}

/// Freeze a [`HashMap`].
///
/// For `K` and `V` where freezing is a no-op, this is a no-op.
impl<K: Freezable, V: Freezable, S: BuildHasher + Default> Freezable
    for HashMap<K, V, S>
where
    K::Frozen: Hash + Eq,
{
    type Frozen = FrozenMap<Frozen<K>, Frozen<V>, S>;

    fn freeze(self) -> Frozen<Self> {
        Frozen(FrozenMap(
            self.into_iter()
                .map(|(k, v)| (k.freeze(), v.freeze()))
                .collect::<HashMap<Frozen<K>, Frozen<V>, S>>(),
        ))
    }
}

impl<K: Hash + Eq, V: Hash, S: BuildHasher> Hash for FrozenMap<K, V, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash = 0;
        for (key, value) in &self.0 {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            value.hash(&mut hasher);
            hash ^= hasher.finish();
        }
        state.write_u64(hash);
    }
}
