pub mod request;
pub mod response;
pub mod tag;

use serde::Serialize;

pub trait Reflect {
    type Output: Serialize;
    fn reflect(self) -> Self::Output;
}

impl<T: Reflect> Reflect for Option<T> {
    type Output = Option<T::Output>;
    fn reflect(self) -> Self::Output {
        self.map(T::reflect)
    }
}

impl<T: Reflect> Reflect for Vec<T> {
    type Output = Vec<T::Output>;
    fn reflect(self) -> Self::Output {
        self.into_iter().map(T::reflect).collect()
    }
}

impl<T1, T2> Reflect for (T1, T2)
where
    T1: Reflect,
    T2: Reflect,
{
    type Output = (T1::Output, T2::Output);
    fn reflect(self) -> Self::Output {
        let (t1, t2) = self;
        (t1.reflect(), t2.reflect())
    }
}

impl<T1, T2, T3> Reflect for (T1, T2, T3)
where
    T1: Reflect,
    T2: Reflect,
    T3: Reflect,
{
    type Output = (T1::Output, T2::Output, T3::Output);
    fn reflect(self) -> Self::Output {
        let (t1, t2, t3) = self;
        (t1.reflect(), t2.reflect(), t3.reflect())
    }
}
