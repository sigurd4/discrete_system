pub trait MISO<Input, Output, const I: usize>
where
    [(); I - 2]:
{
    fn miso(&mut self, x: [Input; I]) -> Output;
}
impl<T, const N: usize> MISO<T, T, N> for [T; N]
where
    [(); N - 2]:,
    T: Copy
{
    fn miso(&mut self, x: [T; N]) -> T
    {
        *self = x;
        *self.last().unwrap()
    }
}