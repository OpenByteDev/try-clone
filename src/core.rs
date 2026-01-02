#[cfg(not(feature = "blanket-impl"))]
use crate::TryClone;

#[cfg(not(feature = "blanket-impl"))]
impl<T: TryClone> TryClone for Option<T> {
    type Error = T::Error;

    fn try_clone(&self) -> Result<Self, Self::Error> {
        self.as_ref().map(|inner| inner.try_clone()).transpose()
    }
}
