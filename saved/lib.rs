pub struct When<T> {
    pub inner: T,
    condition: Box<dyn Fn(&T) -> bool>,
    action: Box<dyn FnMut(&mut T)>,
}

impl<T> When<T> {
    pub fn new(
        inner: T,
        condition: impl Fn(&T) -> bool + 'static,
        action: impl FnMut(&mut T) + 'static,
    ) -> Self {
        println!("When::new()");
        Self {
            inner,
            condition: Box::new(condition),
            action: Box::new(action),
        }
    }
    pub fn lock(&mut self) -> WhenGuard<T> {
        println!("When::lock()");
        WhenGuard { lock: self }
    }
}

impl<T> std::ops::Deref for When<T> {
    type Target = WhenGuard<T>;

    fn deref(&self) -> &Self::Target {
        println!("When::deref()");
        Box::leak(Box::new(WhenGuard {
            lock: self as *const _ as *mut Self,
        }))
    }
}

impl<T> std::ops::DerefMut for When<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("When::deref_mut()");
        Box::leak(Box::new(WhenGuard {
            lock: self as *const _ as *mut Self,
        }))
    }
}

pub struct WhenGuard<T> {
    lock: *mut When<T>,
}

impl<T> std::ops::Deref for WhenGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("WhenGuard::deref()");
        unsafe {
            // Dereference the raw pointer to get a reference to When<T>
            &(*self.lock).inner
        }
    }
}

impl<T> std::ops::DerefMut for WhenGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("WhenGuard::deref_mut()");
        unsafe {
            // Dereference the raw pointer to get a mutable reference to When<T>
            &mut (*self.lock).inner
        }
    }
}

impl<T> Drop for WhenGuard<T> {
    fn drop(&mut self) {
        println!("WhenGuard drop");
        let lock = unsafe { &mut (*self.lock) };
        if (lock.condition)(&lock.inner) {
            (lock.action)(&mut lock.inner)
        }
    }
}

#[inline(always)]
pub fn when<T>(
    t: T,
    condition: impl Fn(&T) -> bool + 'static,
    action: impl Fn(&mut T) + 'static,
) -> When<T> {
    println!("when()");
    When::new(t, condition, action)
}
