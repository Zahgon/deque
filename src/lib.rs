
pub use self::Stolen::*;

use std::sync::Arc;
use std::mem::forget;
use std::ptr;
use std::marker::PhantomData;
use std::cell::Cell;
use std::fmt;

use std::sync::atomic::{AtomicIsize, AtomicPtr, fence};
use std::sync::atomic::Ordering::{SeqCst, Acquire, Release, Relaxed};

static MIN_SIZE: usize = 32;

struct Deque<T: Send> {
    bottom: AtomicIsize,
    top: AtomicIsize,
    array: AtomicPtr<Buffer<T>>,
}

pub struct Worker<T: Send> {
    deque: Arc<Deque<T>>,

    marker: PhantomData<Cell<()>>,
}

pub struct Stealer<T: Send> {
    deque: Arc<Deque<T>>,
}

impl<T: Send> Clone for Stealer<T> {
    fn clone(&self) -> Self { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Debug)]
pub enum Stolen<T> {
    
    Empty,
    
    Abort,
    
    Data(T),
}

struct Buffer<T: Send> {
    storage: *mut T,
    size: usize,
    prev: Option<Box<Buffer<T>>>,
}

pub fn new<T: Send>() -> (Worker<T>, Stealer<T>) { panic!("STUB: not implemented") }

impl<T: Send> Worker<T> {
    
    pub fn push(&self, t: T) { panic!("STUB: not implemented") }
    
    pub fn pop(&self) -> Option<T> { panic!("STUB: not implemented") }
}

impl<T: Send> Stealer<T> {
    
    pub fn steal(&self) -> Stolen<T> { panic!("STUB: not implemented") }
}

impl<T: Send> Deque<T> {
    fn new() -> Deque<T> { panic!("STUB: not implemented") }

    unsafe fn push(&self, data: T) { panic!("STUB: not implemented") }

    unsafe fn pop(&self) -> Option<T> { panic!("STUB: not implemented") }

    unsafe fn steal(&self) -> Stolen<T> { panic!("STUB: not implemented") }
}

impl<T: Send> Drop for Deque<T> {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

#[inline]
unsafe fn take_ptr_from_vec<T>(mut buf: Vec<T>) -> *mut T { panic!("STUB: not implemented") }

#[inline]
unsafe fn allocate<T>(number: usize) -> *mut T { panic!("STUB: not implemented") }

#[inline]
unsafe fn deallocate<T>(ptr: *mut T, number: usize) { panic!("STUB: not implemented") }

impl<T: Send> Buffer<T> {
    unsafe fn new(size: usize) -> Buffer<T> { panic!("STUB: not implemented") }

    fn size(&self) -> isize { panic!("STUB: not implemented") }

    fn mask(&self) -> isize { panic!("STUB: not implemented") }

    unsafe fn elem(&self, i: isize) -> *mut T { panic!("STUB: not implemented") }

    unsafe fn get(&self, i: isize) -> T { panic!("STUB: not implemented") }

    unsafe fn put(&self, i: isize, t: T) { panic!("STUB: not implemented") }

    unsafe fn grow(self: Box<Buffer<T>>, b: isize, t: isize) -> Box<Buffer<T>> { panic!("STUB: not implemented") }
}

impl<T: Send> Drop for Buffer<T> {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

impl<T: Send> fmt::Debug for Deque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<T: Send> fmt::Debug for Worker<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<T: Send> fmt::Debug for Stealer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}
