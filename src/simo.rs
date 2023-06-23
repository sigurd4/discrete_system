pub trait SIMO<Input, Output, const O: usize>
where
    [(); O - 2]:
{
    fn simo(&mut self, x: Input) -> [Output; O];
}
impl<T, const N: usize> SIMO<T, T, N> for [T; N]
where
    [(); N - 2]:,
    T: Copy
{
    fn simo(&mut self, x: T) -> [T; N]
    {
        *unsafe {self.get_unchecked_mut(0)} = x;
        self.rotate_left(1);
        *self
    }
}