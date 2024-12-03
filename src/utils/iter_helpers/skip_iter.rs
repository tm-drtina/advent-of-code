use std::iter::Enumerate;

pub struct SkipIter<'s, I: Iterator> {
    pub(super) parent: Enumerate<I>,
    pub(super) skip: &'s [usize],
}

impl<'s, I> Iterator for SkipIter<'s, I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, next) = self.parent.next()?;
        
        if self.skip.contains(&index) {
            self.next()
        } else {
            Some(next)
        }
    }
}
