use std::cell::RefCell;

thread_local! {
  static ALLOCATOR_INDEX: RefCell<i64> = RefCell::new(0);
}

pub fn alloc(size: usize) -> i64 {
  ALLOCATOR_INDEX.with(|x| *x.borrow_mut() += size as i64);
  ALLOCATOR_INDEX.with(|x| *x.borrow_mut())
}
