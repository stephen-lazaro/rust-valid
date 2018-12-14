#[allow(unused_imports)]
use std::fmt::Debug;
use std::marker::PhantomData;

trait Section <V> {
    type Domain=Self;
    fn to<'a>(self) -> Result<&'a V, String>;
}

impl <R, V> Section <V, Domain=R>
  where R: Section<V> {
    #[allow(dead_code)]
    fn apply<'a>(r: R) -> Result<&'a V, String> {
        r.to()
    }

    #[allow(dead_code)]
    fn apply_unsafe<'a>(r: R) -> &'a V {
        Self::apply(r).unwrap()
    }

    #[allow(dead_code)]
    fn preview<'a>(r: R) -> Option<&'a V> {
        Self::apply(r).ok()
    }
}

trait Modeled <R> {
    type Domain=Self;
    fn from(self) -> R;
}

impl <R, V> Modeled <R, Domain=V>
   where V: Modeled<R> {

   #[allow(dead_code)]
   fn review(v: V) -> R {
       v.from()
   }

   #[allow(dead_code)]
   fn to_link(v: V) -> Link<V, R> {
       Link {
           value: v,
           witness: PhantomData
       }
   }
}

#[derive(Copy, Clone)]
struct Link<V, R> {
    pub value: V,
    witness: PhantomData<R>
}

impl <V, R> ToString for Link<V, R>
  where R: ToString,
        V: Modeled<R, Domain=V>,
        V: Copy
{
      fn to_string(&self) -> String {
         self.clone().value.from().to_string()
      }
}
