mod skip_iter;

pub use skip_iter::SkipIter;

pub trait IterHelpers: Iterator {
    fn skip_iter(self, indicies: &[usize]) -> SkipIter<'_, Self>
    where
        Self: Sized,
    {
        SkipIter {
            parent: self.enumerate(),
            skip: indicies,
        }
    }
}

impl<T> IterHelpers for T where T: Iterator + ?Sized {}
