extern crate num;
use self::num::Integer;

pub fn is_even<A: Integer>(i: A) -> bool
{
    i.is_even()
}

pub fn is_odd<A: Integer>(i: A) -> bool
{
    !i.is_even()
}

pub mod constraint;
pub mod generic;
