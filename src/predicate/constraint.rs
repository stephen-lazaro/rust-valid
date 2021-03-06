#[allow(dead_code)]
use predicate::generic;
use std::marker::PhantomData;

#[allow(dead_code)]
type Predicate <A> = fn(A) -> bool;

#[allow(dead_code)]
pub struct Constraint <F, R, E>
  where F: Fn(R) -> bool
{
  pub run: F,
  pub failure: E,
  pub witness: PhantomData<R>
}

impl<F, R, E> Constraint <F, R, E>
  where F: Fn(R) -> bool
{
    #[allow(dead_code)]
    pub fn run(self, r: R) -> bool {
        let relevant = self.run;
        relevant(r)
    }
}

#[allow(dead_code)]
pub fn contramap <'a, F, P, B, A> (f: F, pred: P) -> impl Fn(B) -> bool
  where F: Fn(B) -> A + 'a,
        P: Fn(A) -> bool + 'a
{
    generic::compose(pred, f)
}

#[allow(dead_code)]
pub fn contramap_constraint <'a, R: 'a, E, S: 'a, F, H> (f: H, constraint: Constraint <F, R, E>) -> Constraint <impl Fn(S) -> bool + 'a, S, E>
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
