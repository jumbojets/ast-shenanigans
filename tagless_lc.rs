type Fun<D, C> = Box<dyn Fn(D) -> C>;

trait Form {
    type Repr<T>;

    fn lam<A, B>(f: Fun<Self::Repr<A>, Self::Repr<B>>) -> Self::Repr<Fun<A, B>>;

    fn appl<A, B>(f: Self::Repr<Fun<A, B>>, x: Self::Repr<A>) -> Self::Repr<B>;
}

struct Eval;

impl Form for Eval {
    type Repr<T> = T;

    fn lam<A, B>(f: Fun<Self::Repr<A>, Self::Repr<B>>) -> Self::Repr<Fun<A, B>> {
        f
    }

    fn appl<A, B>(f: Self::Repr<Fun<A, B>>, x: Self::Repr<A>) -> Self::Repr<B> {
        f(x)
    }
}

struct Length;

impl Form for Length {
    type Repr<T> = u64;

    fn lam<A, B>(f: Fun<Self::Repr<A>, Self::Repr<B>>) -> Self::Repr<Fun<A, B>> {
        f(0) + 1
    }

    fn appl<A, B>(f: Self::Repr<Fun<A, B>>, x: Self::Repr<A>) -> Self::Repr<B> {
        f + x + 1
    }
}

fn test_expr<F: Form>() -> F::Repr<Fun<(), ()>> {
    let id = F::lam::<Fun<(), ()>, Fun<(), ()>>(Box::new(|x| x));
    let id_ = F::lam::<(), ()>(Box::new(|x| x));
    F::appl(id, id_)
}

fn main() {
    dbg!((test_expr::<Eval>())(()));
    dbg!(test_expr::<Length>());
}
