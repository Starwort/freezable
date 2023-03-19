use core::cmp::Ordering;
use core::fmt::{
    self,
    Binary,
    Debug,
    Display,
    Formatter,
    LowerExp,
    LowerHex,
    Octal,
    Pointer,
    UpperExp,
    UpperHex,
};
use core::hash::{Hash, Hasher};
use core::iter::Map;
use core::ops::{
    Add,
    BitAnd,
    BitOr,
    BitXor,
    Deref,
    Div,
    Index,
    Mul,
    Neg,
    RangeBounds,
    Rem,
    Shl,
    Shr,
    Sub,
};

use crate::{Freezable, Frozen, Unfreezable};

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

impl<T: Freezable + ?Sized> IntoIterator for Frozen<T>
where
    T::Frozen: IntoIterator,
{
    type IntoIter = <<T as Freezable>::Frozen as IntoIterator>::IntoIter;
    type Item = <<T as Freezable>::Frozen as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
impl<T: Freezable + ?Sized + Default> Default for Frozen<T> {
    fn default() -> Self {
        T::default().freeze()
    }
}
impl<T: Freezable + ?Sized> Debug for Frozen<T>
where
    T::Frozen: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Frozen({:?})", self.0)
    }
}

macro_rules! impl_fmt {
    ($trait_name:ident) => {
        impl<T: Freezable + ?Sized> $trait_name for Frozen<T>
        where
            T::Frozen: $trait_name,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
impl_fmt!(Display);
impl_fmt!(Octal);
impl_fmt!(LowerHex);
impl_fmt!(UpperHex);
impl_fmt!(Pointer);
impl_fmt!(Binary);
impl_fmt!(LowerExp);
impl_fmt!(UpperExp);

macro_rules! impl_op {
    ($trait_name:ident, $trait_fn:ident) => {
        impl<T: Freezable + ?Sized, U> $trait_name<U> for Frozen<T>
        where
            T::Frozen: $trait_name<U>,
        {
            type Output = <T::Frozen as $trait_name<U>>::Output;

            fn $trait_fn(self, rhs: U) -> Self::Output {
                self.0.$trait_fn(rhs)
            }
        }
        impl<'a, T: Freezable + ?Sized, U> $trait_name<U> for &'a Frozen<T>
        where
            &'a T::Frozen: $trait_name<U>,
        {
            type Output = <&'a T::Frozen as $trait_name<U>>::Output;

            fn $trait_fn(self, rhs: U) -> Self::Output {
                (&self.0).$trait_fn(rhs)
            }
        }
    };
}

macro_rules! impl_ops {
    ($($trait_name:ident ($trait_fn:ident)),* $(,)?) => {
        $(impl_op!($trait_name, $trait_fn);)*
    };
}

impl_ops!(
    Add(add),
    BitAnd(bitand),
    BitOr(bitor),
    BitXor(bitxor),
    Div(div),
    Mul(mul),
    Rem(rem),
    Shl(shl),
    Shr(shr),
    Sub(sub),
);

impl<T: Freezable + ?Sized, U> AsRef<U> for Frozen<T>
where
    T::Frozen: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}

impl<T: Freezable + ?Sized> Neg for Frozen<T>
where
    T::Frozen: Neg,
{
    type Output = <T::Frozen as Neg>::Output;

    fn neg(self) -> Self::Output {
        -self.0
    }
}

impl<T: Freezable + ?Sized, U> RangeBounds<U> for Frozen<T>
where
    T::Frozen: RangeBounds<U>,
{
    fn start_bound(&self) -> core::ops::Bound<&U> {
        self.0.start_bound()
    }

    fn end_bound(&self) -> core::ops::Bound<&U> {
        self.0.end_bound()
    }
}

impl<T: Freezable + ?Sized> Freezable for Frozen<T> {
    type Frozen = T::Frozen;

    fn freeze(self) -> Frozen<Self> {
        Frozen(self.0)
    }
}

impl<T: Freezable + ?Sized> Unfreezable<Frozen<T>> for Frozen<T> {
    fn thaw(wrapped: <Frozen<T> as Freezable>::Frozen) -> Self {
        Frozen(wrapped)
    }
}

pub trait FreezableIteratorExt<T: Freezable>: Iterator<Item = T> + Sized {
    fn frozen(self) -> Map<Self, fn(T) -> Frozen<T>> {
        self.map(Freezable::freeze)
    }
}

impl<T: Freezable, I: Iterator<Item = T>> FreezableIteratorExt<T> for I {
}

pub trait UnfreezableIteratorExt<T: Freezable>:
    Iterator<Item = Frozen<T>> + Sized
{
    fn thawed<U: Unfreezable<T>>(self) -> Map<Self, fn(Frozen<T>) -> U> {
        self.map(Frozen::thaw)
    }
}
impl<T: Freezable, I: Iterator<Item = Frozen<T>>> UnfreezableIteratorExt<T> for I {
}
