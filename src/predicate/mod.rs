extern crate num;
use self::num::Integer;

#[allow(dead_code)]
pub fn is_even<A: Integer>(i: A) -> bool
{
    i.is_even()
}

#[allow(dead_code)]
pub fn is_odd<A: Integer>(i: A) -> bool
{
    !i.is_even()
}

pub mod constraint;
pub mod generic;
