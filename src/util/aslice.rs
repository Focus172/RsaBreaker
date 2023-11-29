use std::{
    alloc::Layout,
    ops::{Index, IndexMut},
    ptr::NonNull,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

#[derive(Debug)]
pub struct AtomicSlice<T> {
    start: AtomicPtr<T>,
    length: AtomicUsize,
}

impl<T> AtomicSlice<T> {
    pub const fn new() -> Self {
        Self {
            start: AtomicPtr::new(NonNull::dangling().as_ptr()),
            length: AtomicUsize::new(0),
        }
    }

    pub fn from_slice(input: &[T]) -> Self {
        let len = input.len();

        let layout = Layout::new::<T>().repeat(len).unwrap().0;
        let p = unsafe { std::alloc::alloc(layout) };

        Self {
            start: AtomicPtr::new(p as *mut T),
            length: AtomicUsize::new(len),
        }
    }

    pub fn len(&self) -> usize {
        self.length.load(Ordering::Relaxed)
    }

    pub fn write(&self, data: &[T]) {
        let oldlen = self.len();
        let newlen = data.len();

        let newlayout = Layout::new::<T>().repeat(newlen).unwrap().0;
        let oldlayout = Layout::new::<T>().repeat(oldlen).unwrap().0;

        let ptr = unsafe { std::alloc::alloc(newlayout) } as *mut T;

        let ptr = self.start.swap(ptr, Ordering::AcqRel) as *mut u8;
        self.length.store(newlen, Ordering::Release);

        unsafe { std::alloc::dealloc(ptr, oldlayout) }
    }
}

impl<T> Index<usize> for AtomicSlice<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.start.load(Ordering::Relaxed).add(index) }
    }
}

impl<T> IndexMut<usize> for AtomicSlice<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.start.load(Ordering::Relaxed).add(index) }
    }
}
