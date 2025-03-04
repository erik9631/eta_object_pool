use crate::errors::PoolError;
pub trait PoolElementProxy<ElementType>
where Self: Drop
{
    type Pool: Pool<ElementType, Proxy = Self>;
    fn new(element: ElementType, pool: &Self::Pool) -> Self;
    fn get(&self) -> &ElementType;
    fn get_mut(&mut self) -> &mut ElementType;
}

pub trait Pool<ElementType> {
    type Proxy: PoolElementProxy<ElementType>;
    fn acquire(&self) -> Option<Self::Proxy>;
    fn push_element(&self, element: ElementType) -> PoolError<()>;
    fn push_elements(&self, elements: Vec<ElementType>) -> PoolError<()>;
}