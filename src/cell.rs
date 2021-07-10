use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Cell<T> {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, val: T) {
        // SAFETY: since Cell can't be shared across threads, there are no concurrent readers -
        // therefore, we have exclusive mutable access.
        unsafe {
            let v = self.value.get();
            *v = val;
        }
    }

    pub fn swap(&self, other: &Cell<T>) {
        // SAFETY: same as Cell<T>.set().
        unsafe { std::ptr::swap(self.value.get(), other.value.get()) }
    }

    pub fn replace(&self, val: T) -> T {
        // SAFETY: same as Cell<T>.set().
        unsafe { std::ptr::replace(self.value.get(), val) }
    }
}

impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        // Safety: since T impl Copy, the value can be copied
        // and returned without duplicating mutable references.
        unsafe {
            let v = self.value.get();
            *v
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn set_get() {
        let c = &Cell::new(5);
        assert!(c.get() == 5);
        c.set(31);
        assert!(c.get() == 31);
        c.set(35);
        assert!(c.get() == 35);
    }
}