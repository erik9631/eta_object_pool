use std::sync::Arc;
use crate::errors::PoolError;
use crate::traits::{Pool, PoolElementProxy};

pub struct ElementProxy<ElementType, PoolType>
where PoolType: Pool<ElementType, Proxy = Self>
{
    element: Option<ElementType>,
    pool_ref: Arc<PoolType> ,
}

impl<ElementType, PoolType> PoolElementProxy<ElementType> for ElementProxy<ElementType, PoolType>
where PoolType: Pool<ElementType, Proxy = Self>
{
    type Pool = PoolType;
    fn new(element: ElementType, pool_ref: Arc<Self::Pool>) -> Self {
        Self {
            element: Some(element),
            pool_ref
        }
    }
    fn get(&self) -> &ElementType {
        &self.element.as_ref().unwrap()
    }
    fn get_mut(&mut self) -> &mut ElementType {
        let element_ref = self.element.as_mut().unwrap();
        element_ref
    }
}

impl<ElementType, PoolType> Drop for ElementProxy<ElementType, PoolType>
where PoolType: Pool<ElementType, Proxy = Self>
{
    fn drop(&mut self) {
        let element = self.element.take().unwrap();
        unsafe{(*self.pool_ref).release(element).expect("Failed to push element")};
    }
}


pub struct FixedPool<ElementType> {
    item_pool: crossbeam::queue::ArrayQueue<ElementType>,
}

impl<ElementType> FixedPool<ElementType> {
    pub fn new(items: Vec<ElementType>) -> PoolError<Arc<Self>> {
        let item_pool = crossbeam::queue::ArrayQueue::new(items.len());
        let pool = FixedPool { item_pool };
        pool.push_elements(items)?;
        Ok(Arc::new(pool))
    }
}

impl<ElementType> Pool<ElementType> for FixedPool<ElementType> {
    type Proxy = ElementProxy<ElementType, Self>;

    fn acquire(self_ref: Arc<Self>) -> Option<Self::Proxy> {
        match self_ref.item_pool.pop() {
            Some(element) => Some(ElementProxy::new(element, self_ref)),
            None => None,
        }
    }
    fn release(&self, element: ElementType) -> PoolError<()> {
        self.item_pool.push(element).map_err(|_| "Failed to push element".to_string())?;
        Ok(())
    }
    fn push_elements(&self, elements: Vec<ElementType>) -> PoolError<()> {
        for element in elements {
            self.item_pool.push(element).map_err(|_| "Failed to push element".to_string())?;
        }
        Ok(())
    }
    fn len(&self) -> usize {
        self.item_pool.len()
    }
}