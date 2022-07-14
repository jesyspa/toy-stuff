use std::sync::atomic::{AtomicUsize, Ordering};
use std::hint;


#[derive(Default)]
pub struct TicketLock {
    next: AtomicUsize,
    active: AtomicUsize,
}

impl TicketLock {
    pub fn new() -> Self { Self::default() }

    pub fn acquire(&self) {
        let ticket = self.next.fetch_add(1, Ordering::SeqCst);
        while let Err(_) = self.active.compare_exchange(ticket, ticket, Ordering::SeqCst, Ordering::SeqCst) {
            hint::spin_loop();
        }
    } 

    pub fn release(&self) {
        self.active.fetch_add(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod test {
    use super::TicketLock;
    use std::cell::UnsafeCell;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_single_thread() {
        let lock = TicketLock::new();
        lock.acquire();
        lock.release();
    }

    #[test]
    fn test_double_release() {
        let lock = TicketLock::new();
        lock.acquire();
        lock.release();
        lock.release();
    }

    struct UnsafeI32(UnsafeCell<i32>);

    unsafe impl Send for UnsafeI32 {} 
    unsafe impl Sync for UnsafeI32 {} 

    impl Default for UnsafeI32 {
        fn default() -> Self { return UnsafeI32(UnsafeCell::new(0)) }
    }

    impl UnsafeI32 {
        unsafe fn inc(&self) {
            *self.0.get() += 1;
        }

        unsafe fn get(&self) -> i32 {
            *self.0.get()
        }
    }

    #[derive(Default)]
    struct TestState {
        lock: TicketLock,
        n: UnsafeI32,
    }

    #[test]
    fn test_multithread_release() {
        let state = Arc::new(TestState::default());
        let mut threads = Vec::new();
        const NUM_THREADS: i32 = 10;
        const NUM_ITERS: i32 = 1000;
        for _ in 0..NUM_THREADS {
            let state_clone = state.clone();
            threads.push(thread::spawn(move|| {
                for _ in 0..NUM_ITERS {
                    state_clone.lock.acquire();
                    unsafe { state_clone.n.inc(); }
                    state_clone.lock.release();
                }
            }))
        }

        for t in threads {
            t.join().expect("Join error.");
        }

        unsafe { assert_eq!(state.n.get(), NUM_THREADS * NUM_ITERS); }
    }
}