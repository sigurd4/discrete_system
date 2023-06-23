use std::collections::VecDeque;

pub trait SISO<Input, Output>
{
    fn siso(&mut self, x: Input) -> Output;
}

impl<T> SISO<T, T> for VecDeque<T>
{
    fn siso(&mut self, x: T) -> T
    {
        self.push_back(x);
        self.pop_front().unwrap()
    }
}
impl<T> SISO<T, Option<T>> for VecDeque<T>
{
    fn siso(&mut self, x: T) -> Option<T>
    {
        let y = self.pop_front();
        self.push_back(x);
        y
    }
}
impl<T> SISO<T, T> for [T; 1]
{
    fn siso(&mut self, x: T) -> T
    {
        x
    }
}
default impl<T, const L: usize> SISO<T, T> for [T; L]
where
    [(); L - 1]:
{
    fn siso(&mut self, mut x: T) -> T
    {
        unsafe {std::mem::swap(self.get_unchecked_mut(0), &mut x)};
        self.rotate_left(1);
        x
    }
}