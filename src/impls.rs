use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, Index};

use crate::{Freezable, Frozen};

impl<T: Freezable + ?Sized> Debug for Frozen<T>
where
    T::Frozen: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Frozen({:?})", self.0)
    }
}

impl<T: Freezable + ?Sized> Deref for Frozen<T> {
    type Target = T::Frozen;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Freezable + ?Sized, U> FromIterator<U> for Frozen<T>
where
    T: FromIterator<U>,
{
    fn from_iter<I: IntoIterator<Item = U>>(iter: I) -> Self {
        iter.into_iter().collect::<T>().freeze()
    }
}

impl<T: Freezable + ?Sized> Hash for Frozen<T>
where
    T::Frozen: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Freezable + ?Sized, U> Index<U> for Frozen<T>
where
    T::Frozen: Index<U>,
{
    type Output = <T::Frozen as Index<U>>::Output;

    fn index(&self, index: U) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Freezable + ?Sized> Clone for Frozen<T>
where
    T::Frozen: Clone,
{
    fn clone(&self) -> Self {
        Frozen(self.0.clone())
    }
}
impl<T: Freezable + ?Sized> Copy for Frozen<T> where T::Frozen: Copy
{
}
impl<T: Freezable + ?Sized> PartialEq for Frozen<T>
where
    T::Frozen: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T: Freezable + ?Sized> Eq for Frozen<T> where T::Frozen: Eq
{
}
impl<T: Freezable + ?Sized> PartialOrd for Frozen<T>
where
    T::Frozen: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T: Freezable + ?Sized> Ord for Frozen<T>
where
    T::Frozen: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl<T: Freezable + ?Sized> Default for Frozen<T>
where
    T: Default,
{
    fn default() -> Self {
        T::default().freeze()
    }
}
