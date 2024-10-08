pub struct When<T> {
    inner: T,
    condition: Box<dyn Fn(&T) -> bool>,
    action: Box<dyn FnMut(&mut T)>,
}

impl<T> When<T> {
    pub fn new(
        inner: T,
        condition: impl Fn(&T) -> bool + 'static,
        action: impl FnMut(&mut T) + 'static,
    ) -> Self {
        // println!("When::new()");
        Self {
            inner,
            condition: Box::new(condition),
            action: Box::new(action),
        }
    }
    pub fn lock(&mut self) -> WhenGuard<T> {
        // println!("When::lock()");
        WhenGuard { lock: self }
    }
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> std::ops::Deref for When<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// I don't know if i keep this as it bypasses the guard system and nullify the goal of the crate
// impl<T> std::ops::DerefMut for When<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }

pub struct WhenGuard<'a, T> {
    lock: &'a mut When<T>,
}

impl<'a, T> std::ops::Deref for WhenGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // println!("WhenGuard::deref()");
        &self.lock.inner
    }
}

impl<'a, T> std::ops::DerefMut for WhenGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // println!("WhenGuard::deref_mut()");
        &mut self.lock.inner
    }
}

impl<'a, T> Drop for WhenGuard<'a, T> {
    fn drop(&mut self) {
        // println!("WhenGuard drop");
        let lock = &mut (*self.lock);
        if (lock.condition)(&lock.inner) {
            (lock.action)(&mut lock.inner)
        }
    }
}

#[macro_export]
macro_rules! when {
    (($condition:expr) $action:block, $var:ident) => {
        let mut $var = ::when::When::new(
            $var,
            |$var: &_| -> bool { $condition },
            |$var: &mut _| $action,
        );
    };
}
