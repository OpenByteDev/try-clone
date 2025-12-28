#[cfg(not(feature = "blanket-impl"))]
use crate::TryClone;

#[cfg(not(feature = "blanket-impl"))]
impl<T: TryClone> TryClone for Option<T> {
    type Err = T::Err;

    fn try_clone(&self) -> Result<Self, Self::Err> {
        self.as_ref().map(|inner| inner.try_clone()).transpose()
    }
}
