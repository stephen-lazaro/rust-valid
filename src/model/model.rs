#[allow(dead_code)]
use std::fmt::Debug;

trait Modeled<V, R, E: ToString + Debug> {
    fn from(&self, v: V) -> R;

    fn to(&self, repr: R) -> Result<V, E>;

    fn apply(&self, r: R) -> Result<V, E> {
        self.to(r)
    }

    fn applyUnsafe(&self, r: R) -> V {
        self.apply(r).unwrap()
    }

    fn review(&self, v: V) -> R {
        self.from(v)
    }

    fn applyOrShow(&self, r: R) -> Result<V, String> {
        match self.apply(r) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string())
        }
    }
}


