use crate::errors::PoolError;
pub trait PoolElementProxy
where Self: Drop
{
    type Element;
    type Pool: Pool<Element = Self::Element, Proxy = Self>;
    fn new(element: Self::Element, pool: &Self::Pool) -> Self;
    fn get(&self) -> &Self::Element;
    fn get_mut(&mut self) -> &mut Self::Element;
}

pub trait Pool {
    type Element;
    type Proxy: PoolElementProxy<Element = Self::Element>;
    fn acquire(&self) -> Option<Self::Proxy>;
    fn push_element(&self, element: Self::Element) -> PoolError<()>;
    fn push_elements(&self, elements: Vec<Self::Element>) -> PoolError<()>;
}