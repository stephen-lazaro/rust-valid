use predicate::generic;
use std::marker::PhantomData;

type Predicate <A> = fn(A) -> bool;

struct Constraint <F, R, E>
  where F: Fn(R) -> bool
{
  run: F, 
  failure: E,
  witness: PhantomData<R>
}

pub fn contramap <B, A> (f: fn(B) -> A, pred: Predicate <A>) -> impl Fn(B) -> bool {
    generic::compose(pred, f)
}

pub fn contramapConstraint <R, E, S, F, G> (f: fn(S) -> R, constraint: Constraint <F, R, E>) -> Constraint <G, S, E> 
    where F: Fn(R) -> bool,
          G: Fn(S) -> bool
{
    Constraint {
        run: |x| constraint.run(f(x)),
        failure: constraint.failure,
        witness: PhantomData
    }
}
