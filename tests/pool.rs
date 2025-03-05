use std::thread;
use std::thread::sleep;
use eta_obj_pool::traits::{Pool, PoolElementProxy};

#[test]
fn acquire_release() {
    let pool = match eta_obj_pool::pool::FixedPool::new(vec![1, 2, 3]) {
        Ok(pool) => pool,
        Err(e) => return assert!(false, "Failed to init pool {}", e),
    };
    let val = Pool::acquire(pool.clone()).unwrap();
    let inner = *val.get();
    assert_eq!(pool.len(), 2, "Invalid pool len");
    assert_eq!(inner, 1, "Failed to get element");
    drop(val);
    assert_eq!(pool.len(), 3, "Invalid pool len");
}
#[test]
fn pool_acquire_till_empty() {
    let pool = match eta_obj_pool::pool::FixedPool::new(vec![1, 2, 3]) {
        Ok(pool) => pool,
        Err(e) => return assert!(false, "Failed to init pool {}", e),
    };
    assert_eq!(pool.len(), 3, "Invalid pool len");
    for i in 0..3 {
        let val = Pool::acquire(pool.clone()).unwrap();
        let inner = *val.get();
        assert_eq!(inner, i + 1, "Failed to get element");
    }
    assert_eq!(pool.len(), 3, "Invalid pool len");
}
#[test]
fn pool_release_invalid() {
    let pool = match eta_obj_pool::pool::FixedPool::new(vec![1, 2, 3]) {
        Ok(pool) => pool,
        Err(e) => return assert!(false, "Failed to init pool {}", e),
    };
    let val = 5;
    match pool.release(val) {
        Ok(_) => assert!(false, "Failed to release element"),
        Err(e) => assert_eq!(e, "Failed to push element".to_string()),
    }
}

#[test]
fn pool_release_parallel() {
    let pool = match eta_obj_pool::pool::FixedPool::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
        Ok(pool) => pool,
        Err(e) => return assert!(false, "Failed to init pool {}", e),
    };
    let thread1_pool = pool.clone();
    thread::spawn(move || {
        let val = Pool::acquire(thread1_pool).unwrap();
        let inner = *val.get();
        assert_eq!(inner, 1, "Failed to get element");
        sleep(std::time::Duration::from_millis(10000));
        drop(val);
    });
    let thread2_pool = pool.clone();
    let res = thread::spawn(move || {
        let val = Pool::acquire(thread2_pool).unwrap();
        let inner = *val.get();
        assert_eq!(inner, 2, "Failed to get element");
        sleep(std::time::Duration::from_millis(10000));
        drop(val);
    });
    drop(pool);
    res.join().unwrap();
}