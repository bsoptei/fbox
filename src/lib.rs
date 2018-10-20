/// The `Apply` trait makes it possible to apply a unary function inside a wrapper. The `apply` method does not take ownership over the wrapper, but it does take ownership over the argument.
pub trait Apply {
    type In;
    type Out;

    fn apply(&self, a: Self::In) -> Self::Out;
}

/// The `ApplyDrop` trait works similarly to `Apply`, except the `apply_drop` method takes ownership over the wrapper.
pub trait ApplyDrop {
    type In;
    type Out;

    fn apply_drop(self, a: Self::In) -> Self::Out;
}

/// `FBox` is a generic wrapper of a unary function. `FBox` lets you compose unary functions via a user friendly syntax and in a type-safe manner.
/// `FBox` works with functions that take ownership over their argument. This way you can exploit the ownership rules of Rust.
/// `FBox` is lazy. It only calls its function when you explicitly tell it to do so. You can call `apply` or `apply_drop` with a single argument, the type of which corresponds to the `FIn` type parameter of the `FBox`. The return type will correspond to the `FOut` type parameter.
pub struct FBox<FIn, FOut> {
    f: Box<Fn(FIn) -> FOut>
}

impl<FIn, FOut> Apply for FBox<FIn, FOut> {
    type In = FIn;
    type Out = FOut;

    fn apply(&self, a: FIn) -> FOut {
        (self.f)(a)
    }
}

impl<FIn, FOut> ApplyDrop for FBox<FIn, FOut> {
    type In = FIn;
    type Out = FOut;

    fn apply_drop(self, a: FIn) -> FOut {
        (self.f)(a)
    }
}

impl<FIn: 'static, FOut: 'static> FBox<FIn, FOut> {
    /// Creates a new `FBox` from a unary function.
    /// # Examples
    ///```
    /// # use fbox::*;
    /// fn inc(n: i32) -> i32 {
    ///     n + 1
    /// }
    ///
    /// let fbox1 = FBox::new(|x| x + 1);
    /// let fbox2 = FBox::new(inc);
    ///
    /// assert_eq!(
    ///     fbox1.apply(3),
    ///     fbox2.apply(3)
    /// );
    ///```
    pub fn new(f: impl Fn(FIn) -> FOut + 'static) -> FBox<FIn, FOut> {
        FBox { f: Box::new(f) }
    }

    /// You can compose a function `f(x)` inside an `FBox` with another function `g`. The result is a new function `f(g(x))` wrapped in a new `FBox`. The output type of `g` must match the input type of `f`.
    /// # Examples
    ///```
    /// # use fbox::*;
    /// assert_eq!(
    ///     10,
    ///     FBox::new(|x| x + 1).compose(|x| x * x).apply(3),
    /// );
    ///```
    pub fn compose<GIn: 'static>(self, g: impl Fn(GIn) -> FIn + 'static) -> FBox<GIn, FOut> {
        FBox::new(move |x| (self.f)(g(x)))
    }

    /// Similar to `compose`, except the result is `g(f(x))` in a new `FBox`. The output type of `f` must match the input type of `g`.
    /// # Examples
    ///```
    /// # use fbox::*;
    /// assert_eq!(
    ///     16,
    ///     FBox::new(|x| x + 1).and_then(|x| x * x).apply(3),
    /// );
    ///```
    pub fn and_then<GOut: 'static>(self, g: impl Fn(FOut) -> GOut + 'static) -> FBox<FIn, GOut> {
        FBox::new(move |x| g((self.f)(x)))
    }

    /// Similar to `compose`, except its argument is another `FBox` with a function `g`.
    /// # Examples
    ///```
    /// # use fbox::*;
    /// assert_eq!(
    ///     10,
    ///     FBox::new(|x| x + 1).compose_b(FBox::new(|x| x * x)).apply(3)
    /// );
    ///```
    pub fn compose_b<GIn: 'static>(self, other: FBox<GIn, FIn>) -> FBox<GIn, FOut> {
        FBox::new(move |x| (self.f)((other.f)(x)))
    }

    /// Similar to `and_then`, except its argument is another `FBox` with a function `g`.
    /// # Examples
    ///```
    /// # use fbox::*;
    /// assert_eq!(
    ///     16,
    ///     FBox::new(|x| x + 1).and_then_b(FBox::new(|x| x * x)).apply(3)
    /// );
    ///```
    pub fn and_then_b<GOut: 'static>(self, other: FBox<FOut, GOut>) -> FBox<FIn, GOut> {
        FBox::new(move |x| (other.f)((self.f)(x)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_and_apply_drop() {
        let fb1 = FBox::new(|x| x + 1);

        assert_eq!(
            fb1.apply(3),
            fb1.apply_drop(3)
        );
    }
}