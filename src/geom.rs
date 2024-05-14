pub trait BoardGeom {
    fn rotate(&mut self);
    fn size(&self) -> (usize, usize);
    #[inline]
    fn rows(&self) -> usize {
        self.size().0
    }
    #[inline]
    fn cols(&self) -> usize {
        self.size().1
    }
}
