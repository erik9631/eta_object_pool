use std::thread;
use std::thread::sleep;
use eta_obj_pool::traits::{Pool, PoolElementProxy};

#[test]
fn acquire_release() {
    let pool = match eta_obj_pool::pool::SegPool::new(vec![1, 2, 3]) {
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
    let pool = match eta_obj_pool::pool::SegPool::new(vec![1, 2, 3]) {
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
fn pool_release_parallel() {
    let pool = match eta_obj_pool::pool::SegPool::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
        Ok(pool) => pool,
        Err(e) => return assert!(false, "Failed to init pool {}", e),
    };
    let thread1_pool = pool.clone();
    thread::spawn(move || {
        let val = Pool::acquire(thread1_pool).unwrap();
        let inner = *val.get();
        assert_eq!(inner, inner, "Failed to get element");
        sleep(std::time::Duration::from_millis(10000));
        drop(val);
    });
    let thread2_pool = pool.clone();
    let res = thread::spawn(move || {
        let val = Pool::acquire(thread2_pool).unwrap();
        let inner = *val.get();
        assert_eq!(inner, inner, "Failed to get element");
        sleep(std::time::Duration::from_millis(10000));
        drop(val);
    });
    drop(pool);
    res.join().unwrap();
}
#[test]
fn pool_push_dynamic() {
    let pool = match eta_obj_pool::pool::SegPool::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
        Ok(pool) => pool,
        Err(e) => return assert!(false, "Failed to init pool {}", e),
    };

    let pool_thread1 = pool.clone();
    let handle = thread::spawn(move || {
        pool_thread1.push_elements(vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]).unwrap();

        for _ in 0..10 {
            let val = Pool::acquire(pool_thread1.clone()).unwrap();
            let inner = *val.get();
            assert_eq!(inner, inner, "Failed to get element");
        }
    });

    let pool_thread2 = pool.clone();
    let handle2 = thread::spawn(move || {
        pool_thread2.push_elements(vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30]).unwrap();
        for _ in 0..10 {
            let val = Pool::acquire(pool_thread2.clone()).unwrap();
            let inner = *val.get();
            assert_eq!(inner, inner, "Failed to get element");
        }
    });
    handle2.join().unwrap();
    handle.join().unwrap();

    assert_eq!(pool.len(), 30, "Invalid pool len");

}