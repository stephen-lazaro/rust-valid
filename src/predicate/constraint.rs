use predicate::generic;
use std::marker::PhantomData;

type Predicate <A> = fn(A) -> bool;

pub struct Constraint <F, R, E>
  where F: Fn(R) -> bool
{
  run: F, 
  failure: E,
  witness: PhantomData<R>
}

pub fn contramap <'a, F, P, B, A> (f: F, pred: P) -> impl Fn(B) -> bool
  where F: Fn(B) -> A + 'a,
        P: Fn(A) -> bool + 'a
{
    generic::compose(pred, f)
}

pub fn contramapConstraint <'a, R: 'a, E, S: 'a, F, G, H> (f: H, constraint: Constraint <F, R, E>) -> Constraint <impl Fn(S) -> bool + 'a, S, E> 
    where F: Fn(R) -> bool + 'a,
          H: Fn(S) -> R + 'a
{
    let composed = contramap(f, constraint.run);
    Constraint {
        run: composed,
        failure: constraint.failure,
        witness: PhantomData
    }
}
