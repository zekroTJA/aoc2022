#[allow(clippy::len_without_is_empty)]
pub trait Vector
where
    Self::Output: Vector,
{
    type Output;

    fn len(&self) -> f64;
    fn flatten(&self) -> Self::Output;
}
