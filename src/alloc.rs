#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

#[cfg(any(feature = "blanket-impl", feature = "nightly"))]
use crate::TryClone;

#[cfg(feature = "nightly")]
use alloc::{
    alloc::Allocator,
    boxed::Box,
    collections::{BinaryHeap, TryReserveError, VecDeque},
    rc::Rc,
    sync::Arc,
    vec::Vec,
};

#[cfg(feature = "nightly")]
use alloc::alloc::AllocError;

#[cfg(feature = "blanket-impl")]
use crate::ForwardTryCloneToClone;

#[cfg(feature = "blanket-impl")]
impl<T: ?Sized, A> !ForwardTryCloneToClone for Box<T, A> {}
#[cfg(feature = "nightly")]
#[cfg(not(feature = "blanket-impl"))]
impl<T: Clone, A: Allocator + Clone> TryClone for Box<T, A> {
    type Err = AllocError;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        let alloc = Self::allocator(self).clone();
        Self::try_clone_from_ref_in(self, alloc)
    }
}

#[cfg(feature = "blanket-impl")]
impl<T: ?Sized, A> !ForwardTryCloneToClone for Arc<T, A> {}
#[cfg(feature = "nightly")]
impl<T: Clone, A: Allocator + Clone> TryClone for Arc<T, A> {
    type Err = AllocError;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        let alloc = Self::allocator(self).clone();
        Self::try_clone_from_ref_in(self, alloc)
    }
}

#[cfg(feature = "blanket-impl")]
impl<T: ?Sized, A> !ForwardTryCloneToClone for Rc<T, A> {}
#[cfg(feature = "nightly")]
impl<T: Clone, A: Allocator + Clone> TryClone for Rc<T, A> {
    type Err = AllocError;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        let alloc = Self::allocator(self).clone();
        Self::try_clone_from_ref_in(self, alloc)
    }
}

#[cfg(feature = "blanket-impl")]
impl<T: ?Sized, A> !ForwardTryCloneToClone for Vec<T, A> {}
#[cfg(feature = "nightly")]
impl<T: Clone, A: Allocator + Clone> TryClone for Vec<T, A> {
    type Err = TryReserveError;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        let alloc = self.allocator().clone();
        let mut cloned = Self::try_with_capacity_in(self.len(), alloc)?;
        cloned.extend(self.iter().cloned());
        Ok(cloned)
    }
}

#[cfg(feature = "blanket-impl")]
impl<T: ?Sized, A> !ForwardTryCloneToClone for VecDeque<T, A> {}
#[cfg(feature = "nightly")]
impl<T: Clone, A: Allocator + Clone> TryClone for VecDeque<T, A> {
    type Err = TryReserveError;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        let alloc = self.allocator().clone();
        let mut cloned = Self::new_in(alloc);
        cloned.try_reserve(self.len())?;
        cloned.extend(self.iter().cloned());
        Ok(cloned)
    }
}

#[cfg(feature = "blanket-impl")]
impl<T: ?Sized, A> !ForwardTryCloneToClone for BinaryHeap<T, A> {}
#[cfg(feature = "nightly")]
impl<T: Clone + Ord, A: Allocator + Clone> TryClone for BinaryHeap<T, A> {
    type Err = TryReserveError;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        let alloc = self.allocator().clone();
        let mut cloned = Self::new_in(alloc);
        cloned.try_reserve(self.len())?;
        cloned.extend(self.iter().cloned());
        Ok(cloned)
    }
}
