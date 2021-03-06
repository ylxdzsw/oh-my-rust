pub trait MonadExt {
    fn ignore(&self) {}
}

pub trait SizedMonadExt: Sized {
    fn apply<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self); self
    }
}

pub trait ResultExt {
    type S;
    fn msg<T>(self, x: T) -> Result<Self::S, T>;
}

pub trait PrintableResultExt {
    fn warn(self) -> Self;
}

pub trait UnwrapUnchecked {
    type O;
    unsafe fn unwrap_unchecked(self) -> Self::O;
}