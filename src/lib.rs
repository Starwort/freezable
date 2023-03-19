#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]
mod frozen_core;
#[cfg(feature = "std")]
mod frozen_std;
mod impls;
pub mod prelude;
pub use impls::{FreezableIteratorExt, UnfreezableIteratorExt};

/// The primary trait for the `freezable` crate.
///
/// This trait is implemented for all types that can be frozen, and indicates
/// the result of freezing this type.
///
/// It is a logic error for a frozen type to have interior mutability.
pub trait Freezable {
    /// The concrete type that is returned when freezing this type.
    type Frozen;

    /// Freeze this type.
    fn freeze(self) -> Frozen<Self>;
}

/// Unfreeze some frozen type. This is implemented for types that can be
/// unfrozen, and indicates the frozen type that they unfreeze. A frozen type
/// may be unfreezable to multiple types, and multiple types may unfreeze the
/// same frozen type
pub trait Unfreezable<T: Freezable + ?Sized> {
    /// Unfreeze the frozen type.
    fn thaw(wrapped: T::Frozen) -> Self;
}

/// A frozen `T`.
///
/// This type is returned by the `Freezable::freeze` method, and is a standard
/// wrapper around `T::Frozen`.
#[repr(transparent)]
pub struct Frozen<T: Freezable + ?Sized>(T::Frozen);
impl<T: Freezable + ?Sized> Frozen<T> {
    /// Construct a new `Frozen` from a frozen value.
    ///
    /// Required for implementing [`Unfreezable`] and [`Freezable`] for
    /// a type - not usually useful for user code.
    pub fn new(frozen: T::Frozen) -> Self {
        Frozen(frozen)
    }

    /// Unfreeze this type into some compatible `U`.
    pub fn thaw<U>(self) -> U
    where
        U: Unfreezable<T>,
    {
        <U as Unfreezable<T>>::thaw(self.0)
    }
}

macro_rules! impl_self_freezable {
    ($impl_type:ty $(, $($lifetime_params:lifetime),*)?) => {
        impl $(<$($lifetime_params),*>)? Freezable for $impl_type {
            type Frozen = Self;

            fn freeze(self) -> Frozen<Self> {
                Frozen(self)
            }
        }
        impl $(<$($lifetime_params),*>)? Unfreezable<$impl_type> for $impl_type {
            fn thaw(wrapped: <Self as Freezable>::Frozen) -> Self {
                wrapped
            }
        }
    };
}

pub(crate) use impl_self_freezable;
