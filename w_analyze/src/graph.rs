///! This module implements a gc system for implementing a graph data structure.
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

// Failed experiment :c
fn test() {
    let mut a = Gc::default();
    let mut b = Gc::default();

    let v = GcRef::new(&mut a, ());

    v.write(&mut b);
}

#[derive(Default)]
pub struct Gc {
    _items: Vec<GcOwned>,
}

pub struct GcRef<'gc, T> {
    inner: GcShared<UnsafeCell<T>>,
    _phantom: PhantomData<&'gc ()>,
}

pub struct GcRefReadGuard<'a, 'gc, T> {
    inner: &'a GcRef<'gc, T>,
    _gc: &'gc Gc,
}

pub struct GcRefWriteGuard<'a, 'gc, T> {
    inner: &'a GcRef<'gc, T>,
    _gc: &'gc mut Gc,
}

struct GcOwned {
    // () == void
    ptr: Option<NonNull<()>>,
    v_drop: fn(NonNull<()>),
}

struct GcShared<T> {
    ptr: NonNull<T>,
    _phantom: PhantomData<T>,
}

impl<'gc, T> GcRef<'gc, T> {
    pub fn new(gc: &'gc mut Gc, t: T) -> Self {
        let (owned, shared) = share(UnsafeCell::new(t));
        gc._items.push(owned);

        Self {
            inner: shared,
            _phantom: Default::default()
        }
    }

    pub fn read(&self, gc: &'gc Gc) -> GcRefReadGuard<'_, 'gc, T> {
        GcRefReadGuard {
            inner: self,
            _gc: gc,
        }
    }

    pub fn write(&self, gc: &'gc mut Gc) -> GcRefWriteGuard<'_, 'gc, T> {
        GcRefWriteGuard {
            inner: self,
            _gc: gc,
        }
    }
}


fn share<T>(t: T) -> (GcOwned, GcShared<T>) {
    let bx = Box::new(t);
    let raw = NonNull::new(Box::into_raw(bx)).unwrap();
    (
        GcOwned {
            ptr: Some(unsafe { mem::transmute(raw) }),
            v_drop: |ptr| unsafe { drop(Box::from_raw(transmute::<*mut _, *mut T>(ptr.as_ptr()))) },
        },
        GcShared {
            ptr: raw,
            _phantom: Default::default(),
        },
    )
}


impl Drop for GcOwned {
    fn drop(&mut self) {
        (self.v_drop)(self.ptr.take().expect("double drop"))
    }
}

impl<T> Deref for GcShared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}


impl<'a, 'gc, T> Deref for GcRefReadGuard<'a, 'gc, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.inner.get() }
    }
}


impl<'a, 'gc, T> Deref for GcRefWriteGuard<'a, 'gc, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.inner.get() }
    }
}

impl<'a, 'gc, T> DerefMut for GcRefWriteGuard<'a, 'gc, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.inner.get() }
    }
}
