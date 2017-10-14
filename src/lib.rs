#[allow(unused_imports)]
#[macro_use]
extern crate quickcheck;

pub mod error {
    #[allow(unused_imports)]
    use std::cmp::max;
    #[allow(unused_imports)]
    use quickcheck;

    pub enum Expression <A> {
        Pure(A),
        Not(Box <Expression <A>>),
        And(Box <Expression <A>>, Box < Expression <A>>),
        Or(Box <Expression <A>>, Box <Expression <A>>),
        Context(String, Box <Expression <A>>),
    }

    pub fn of <A> (x: A) -> Expression <A> {
        Expression::Pure(x)
    }

    pub fn not <A> (x: Expression <A>) -> Expression <A> {
        Expression::Not(Box::new(x))
    }

    pub fn and <A> (x: Expression <A>, y: Expression <A>) -> Expression <A> {
        Expression::And(Box::new(x), Box::new(y))
    }

    pub fn or <A> (x: Expression <A>, y: Expression <A>) -> Expression <A> {
        Expression::Or(Box::new(x), Box::new(y))
    }
    
    pub fn cata (x: Expression <String>) -> String {
        match x {
            Expression::Pure (x) => x,
            Expression::Not (negated) => format!("Not ({})", cata (*negated)),
            Expression::And (left, right) => format!("({}) and ({})", cata (*left), cata (*right)),
            Expression::Or (left, right) => format!("({}) or ({})", cata (*left), cata (*right)),
            Expression::Context (ctx, rem) => format!("({}) of [{}]", ctx, cata (*rem))
        }
    }
}

pub mod generic {
    /*trait Profunctor {
        type F;
        type Source;
        type Sink;
        // Suspect any implementation would have to use associated types
        fn dimap <G, C, D> (self: Self, f: fn(C) -> Source, g: fn(B) -> D) -> G
            where G::Source = C
                  G::Sink = D;
    }
    screwed by lack of HKTs
    */
}

pub mod predicate {
    use error::Expression;
    struct Predicate <A> { run: fn(A) -> Expression <String> }

    pub fn contramap <B, A> (f: fn(B) -> A, pred: Predicate <A>) -> Predicate <B> {
        Predicate <B> { run = pred.run.compose(f) }
    }
}


#[cfg(test)]
mod tests {
    use error;
    use error::not;
    use error::cata;

    quickcheck! {
        fn cata_works_pure(str: String) -> bool {
            cata(error::of(str.clone())) == str
        }
    }
    
    quickcheck! {
        fn cata_works_not(str: String) -> bool {
            cata(
                not(
                    error::of(
                        str.clone()
                    )
                )
            ) == format!("Not ({})", str)
        }
    }
}
