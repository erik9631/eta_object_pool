use std::sync::Arc;
use crate::errors::PoolError;
pub trait PoolElementProxy<ElementType>
where Self: Drop
{
    type Pool: Pool<ElementType, Proxy = Self>;
    fn new(element: ElementType, pool: Arc<Self::Pool>) -> Self;
    fn get(&self) -> &ElementType;
    fn get_mut(&mut self) -> &mut ElementType;
}

pub trait Pool<ElementType> {
    type Proxy: PoolElementProxy<ElementType>;
    fn acquire(self_ref: Arc<Self>) -> Option<Self::Proxy>;
    fn release(&self, element: ElementType) -> PoolError<()>;
    fn push_elements(&self, elements: Vec<ElementType>) -> PoolError<()>;
    fn len(&self) -> usize;
}