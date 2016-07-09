#![allow(dead_code)]
use alloc::heap;
use core::isize;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::ptr::Unique;
use core::slice;
use libc;

struct RawVec<T> {
    ptr: Unique<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        // assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        unsafe { RawVec { ptr: Unique::new(0x01 as *mut _), cap: 0 } }
    }

    fn grow(&mut self) {
        unsafe {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            let (new_cap, ptr) = if self.cap == 0 {
                let ptr = heap::allocate(elem_size, align);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * elem_size;
                // assert!(old_num_bytes <= (isize::MAX as usize) / 2, "capacity overflow");
                let new_num_bytes = old_num_bytes * 2;
                let ptr = heap::reallocate(*self.ptr as *mut _, old_num_bytes, new_num_bytes, align);
                (new_cap, ptr)
            };

            if ptr.is_null() { oom(); }

            self.ptr = Unique::new(ptr as *mut _);
            self.cap = new_cap;
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let num_bytes = elem_size * self.cap;
            unsafe { heap::deallocate(*self.ptr as *mut _, num_bytes, align); }
        }
    }
}

pub struct Vec<T> {
    buf: RawVec<T>,
    len: usize
}

impl<T> Vec<T> {
    fn ptr(&self) -> *mut T { *self.buf.ptr }
    fn cap(&self) -> usize { self.buf.cap }

    pub fn new() -> Self {
        Vec { buf: RawVec::new(), len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() { self.buf.grow(); }
        unsafe { ptr::write(self.ptr().offset(self.len as isize), elem); }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().offset(self.len as isize))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        // assert!(index <= self.len, "index out of bounds");
        if self.cap() == self.len { self.buf.grow(); }
        unsafe {
            if index < self.len {
                ptr::copy(self.ptr().offset(index as isize),
                          self.ptr().offset(index as isize + 1),
                          self.len - index);
            }
            ptr::write(self.ptr().offset(index as isize), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        // assert!(index < self.len, "index out of bounds");
        self.len -=1;
        unsafe {
            let result = ptr::read(self.ptr().offset(index as isize));
            ptr::copy(self.ptr().offset(index as isize + 1),
                      self.ptr().offset(index as isize),
                      self.len - index);
            result
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

struct IntoIter<T> {
    _buf: RawVec<T>,
    start: *const T,
    end: *const T
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> Vec<T> {
    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let buf = ptr::read(&self.buf);
            let len = self.len;
            mem::forget(self);
            IntoIter {
                start: *buf.ptr,
                end: if buf.cap == 0 { *buf.ptr } else { buf.ptr.offset(len as isize) },
                _buf: buf
            }
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}



fn oom() -> ! { unsafe { libc::exit(-9999 as libc::c_int); } }
