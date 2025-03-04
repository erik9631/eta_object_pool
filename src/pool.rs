use crate::errors::PoolError;
use crate::traits::{Pool, PoolElementProxy};

pub struct PoolElement<ElementType, PoolType>
where PoolType: Pool<Element = ElementType, Proxy = Self>
{
    element: Option<ElementType>,
    pool_ptr: *const PoolType ,
}

impl<ElementType, PoolType> PoolElementProxy for PoolElement<ElementType, PoolType>
where PoolType: Pool<Element = ElementType, Proxy = Self>
{
    type Element = ElementType;
    type Pool = PoolType;
    fn new(element: Self::Element, pool_ref: &Self::Pool) -> Self {
        Self {
            element: Some(element),
            pool_ptr: pool_ref
        }
    }

    fn get(&self) -> &Self::Element {
        &self.element.as_ref().unwrap()
    }

    fn get_mut(&mut self) -> &mut Self::Element {
        let element_ref = self.element.as_mut().unwrap();
        element_ref
    }
}

impl<ElementType, PoolType> Drop for PoolElement<ElementType, PoolType>
where PoolType: Pool<Element = ElementType, Proxy = Self>
{
    fn drop(&mut self) {
        let element = self.element.take().unwrap();
        unsafe{(*self.pool_ptr).push_element(element).expect("Failed to push element")};
    }
}


pub struct FixedPool<ElementType> {
    item_pool: crossbeam::queue::ArrayQueue<ElementType>,
}

impl<ElementType> FixedPool<ElementType> {
    pub fn new(items: Vec<ElementType>) -> PoolError<Self> {
        let item_pool = crossbeam::queue::ArrayQueue::new(items.len());
        let pool = FixedPool { item_pool };
        pool.push_elements(items)?;
        Ok(pool)
    }
}

impl<ElementType> Pool for FixedPool<ElementType> {
    type Queue = crossbeam::queue::ArrayQueue<ElementType>;
    type Element = ElementType;
    type Proxy = PoolElement<ElementType, Self>;

    fn acquire(&self) -> Option<Self::Proxy> {
        match self.item_pool.pop() {
            Some(element) => Some(PoolElement::new(element, &self)),
            None => None,
        }
    }

    fn push_element(&self, element: ElementType) -> PoolError<()> {
        self.item_pool.push(element).map_err(|_| "Failed to push element".to_string())?;
        Ok(())
    }

    fn push_elements(&self, elements: Vec<ElementType>) -> PoolError<()> {
        for element in elements {
            self.item_pool.push(element).map_err(|_| "Failed to push element".to_string())?;
        }
        Ok(())
    }
}