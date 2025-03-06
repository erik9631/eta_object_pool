## ETA POOL

A simple thread safe object pool implementation in Rust.

### Usage

```rust
use eta_obj_pool::pool::FixedPool;

let pool = FixedPool::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).unwrap(); /// Or any vector of elements

for _ in 0..10 {
    let element = pool.acquire().unwrap();
    println!("{}", element.get());
}
```

The objects acquired from the proxies are automatically returned to the pool when they are dropped.

