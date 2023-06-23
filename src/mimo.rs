pub trait MIMO<Input, Output, const I: usize, const O: usize>
where
    [(); I - 2]:,
    [(); O - 2]:
{
    fn mimo(&mut self, x: [Input; I]) -> [Output; O];
}
impl<T, const L: usize> MIMO<T, T, L, L> for [T; L]
where
    [(); L - 2]:,
    T: Copy
{
    fn mimo(&mut self, x: [T; L]) -> [T; L]
    {
        *self = x;
        *self
    }
}