#![cfg_attr(
    feature = "blanket-impl",
    feature(auto_traits, negative_impls, min_specialization)
)]
#![cfg_attr(
    all(feature = "alloc", feature = "nightly"),
    feature(allocator_api, clone_from_ref)
)]
#![cfg_attr(feature = "doc-cfg", feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! Fallible cloning.
//!
//! This crate defines [`TryClone`], a small trait for types whose cloning
//! operation can fail. Unlike [`Clone`], which is infallible by design,
//! [`TryClone`] returns a [`Result`] to allow implementations to report errors.
//!
//! Implementations are provided for standard library types that expose a
//! `try_clone` API, such as [`File`](::std::fs::File), as well as for common containers
//! and collections ([`Box`], [`Arc`](::std::sync::Arc), [`Vec`], ...) where appropriate APIs
//! exists.
//!
//! A blanket implementation is available behind the `blanket-impl` feature,
//! which implements [`TryClone`] for all [`Clone`] types.

/// A fallible variant of [`Clone`].
///
/// this trait is intended for types where cloning can fail, for example due to
/// allocation failures, resource limits, or external constraints.
///
/// Unlike [`Clone`], the cloning operation returns a [`Result`] with an
/// associated error type.
pub trait TryClone: Sized {
    /// The error type returned when cloning fails.
    type Err;

    /// Tries to create a duplicate of the value.
    ///
    /// # Errors
    /// Returns [`Err`] if cloning fails.
    fn try_clone(&self) -> Result<Self, Self::Err>;

    /// Performs copy-assignment from source.
    /// `a.clone_from(&b)` is equivalent to `a = b.clone()` in functionality, but can be overridden to reuse the resources of `a` to avoid unnecessary allocations.
    ///
    /// # Errors
    /// Returns [`Err`] if cloning fails.
    fn try_clone_from(&mut self, source: &Self) -> Result<(), Self::Err> {
        *self = source.try_clone()?;
        Ok(())
    }
}

/// A trait for fallibly cloning a value into an owned form.
///
/// This is similar in spirit to [`ToOwned`], but allows the conversion
/// to fail and does not require `Owned` to implement [`Clone`].
pub trait TryCloneToOwned: Sized {
    /// The owned type produced by cloning.
    type Owned;
    /// The error type returned when cloning fails.
    type Err;

    /// Attempts to create an owned version of the value.
    ///
    /// # Errors
    /// Returns [`Err`] if cloning fails.
    fn try_clone_to_owned(&self) -> Result<Self::Owned, Self::Err>;
}

#[cfg(feature = "blanket-impl")]
use ::core::convert::Infallible;

#[cfg(feature = "blanket-impl")]
include!("blanket_impl.rs");

#[cfg(feature = "blanket-impl")]
impl<T: Clone + ForwardTryCloneToClone> TryClone for T {
    type Err = Infallible;

    default fn try_clone(&self) -> Result<Self, Self::Err> {
        Ok(self.clone())
    }

    default fn try_clone_from(&mut self, source: &Self) -> Result<(), Self::Err> {
        Ok(self.clone_from(source))
    }
}

mod core;

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "std")]
mod std;

#[cfg(all(feature = "std", windows))]
mod windows;

#[cfg(all(feature = "std", unix))]
mod unix;
