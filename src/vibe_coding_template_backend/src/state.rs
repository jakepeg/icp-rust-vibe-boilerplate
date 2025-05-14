use std::cell::RefCell;

thread_local! {
    pub static COUNTER: RefCell<u64> = RefCell::new(0);
}
