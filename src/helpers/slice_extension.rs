pub trait SliceExtension<T: PartialOrd> {
    fn swap_if_needed(&mut self, i: usize, j: usize);
}

impl<T> SliceExtension<T> for [T]
where
    T: PartialOrd,
{
    #[inline]
    fn swap_if_needed(&mut self, i: usize, j: usize) {
        if i == j {
            return;
        }

        self.swap(i, j);
    }
}
