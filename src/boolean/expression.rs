#[allow(unused_imports)]
use std::cmp::max;
#[allow(unused_imports)]
use quickcheck;

#[allow(dead_code)]
pub enum Expression <A> {
    Pure(A),
    Not(Box <Expression <A>>),
    And(Box <Expression <A>>, Box < Expression <A>>),
    Or(Box <Expression <A>>, Box <Expression <A>>),
    Context(String, Box <Expression <A>>),
}

#[allow(dead_code)]
pub fn of <A> (x: A) -> Expression <A> {
    Expression::Pure(x)
}

#[allow(dead_code)]
pub fn not <A> (x: Expression <A>) -> Expression <A> {
    Expression::Not(Box::new(x))
}

#[allow(dead_code)]
pub fn and <A> (x: Expression <A>, y: Expression <A>) -> Expression <A> {
    Expression::And(Box::new(x), Box::new(y))
}

#[allow(dead_code)]
pub fn or <A> (x: Expression <A>, y: Expression <A>) -> Expression <A> {
    Expression::Or(Box::new(x), Box::new(y))
}

#[allow(dead_code)]
pub fn context <A> (ctx: String, x: Expression <A>) -> Expression <A> {
    Expression::Context(ctx, Box::new(x))
}

#[allow(dead_code)]
pub fn cata (x: Expression <String>) -> String {
    match x {
        Expression::Pure (x) => x,
        Expression::Not (negated) => format!("Not ({})", cata (*negated)),
        Expression::And (left, right) => format!("({}) and ({})", cata (*left), cata (*right)),
        Expression::Or (left, right) => format!("({}) or ({})", cata (*left), cata (*right)),
        Expression::Context (ctx, rem) => format!("({}) of [{}]", ctx, cata (*rem))
    }
}
