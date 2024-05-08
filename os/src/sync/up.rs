use core::cell::{RefCell, RefMut};
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

// 为UPSafeCell实现Sync, 允许单线程程序访问这个全局变量
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
