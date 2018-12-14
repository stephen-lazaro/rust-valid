#![feature(associated_type_defaults)]
#[allow(unused_imports)]
#[macro_use]
extern crate quickcheck;

mod boolean;
mod predicate;
mod model;

#[allow(dead_code)]
pub type Validation<V, R, E> = boolean::expression::Expression<predicate::constraint::Constraint<V, R, E>>;

#[cfg(test)]
mod tests {
    use boolean::expression::not;
    use boolean::expression::cata;
    use boolean::expression::of;
    use boolean::expression::or;
    use boolean::expression::and;
    use boolean::expression::context;
    #[allow(unused_imports)]
    use boolean::expression::Expression;
    use predicate::generic::compose;
    use predicate::constraint::Constraint;
    #[allow(unused_imports)]
    use predicate::constraint::contramap;
    use predicate::constraint::contramap_constraint;
    use std::marker::PhantomData;


    quickcheck! {
        fn cata_works_pure(str: String) -> bool {
            cata(of(str.clone())) == str
        }
    }

    quickcheck! {
        fn cata_works_not_simple(str: String) -> bool {
            cata(
                not(
                    of(
                        str.clone()
                    )
                )
            ) == format!("Not ({})", str)
        }
    }

    quickcheck! {
        fn cata_works_simple_or(strs: (String, String)) -> bool {
            let (str, str_) = strs;
            cata(
                or(
                    of(
                        str.clone())
                    ,
                    of(
                        str_.clone()
                        )
                    )
                ) == format!("({}) or ({})", str, str_)
        }
    }

    quickcheck! {
       fn cata_works_simple_and(strs: (String, String)) -> bool {
            let (str, str_) = strs;
            cata(
                and(
                    of(
                        str.clone())
                    ,
                    of(
                        str_.clone()
                        )
                    )
                ) == format!("({}) and ({})", str, str_)
        }
    }

    quickcheck! {
        fn cata_works_simple_context(strs: (String, String)) -> bool {
            let (str, str_) = strs;
            cata(
                context(
                    str.clone()
                    ,
                    of(
                        str_.clone()
                        )
                    )
                ) == format!("({}) of [{}]", str, str_)
        }
    }

    #[test]
    fn composition_works () {
        let times_two = |x: i32| x * 2;
        let add_five = |y: i32| y + 5;

        let composed = compose(add_five, times_two);
        assert!(composed(5) == 15)
    }

    #[test]
    fn composition_works_2 () {
        let times_fifty_five = |x: i32| x * 55;
        let add_seven = |y: i32| y + 7;

        let composed = compose(add_seven, times_fifty_five);
        assert!(composed(1) == 62)
    }

    #[test]
    fn contramap_constraint_test () {
        let constraint = Constraint {
            run: |x: i32| x == 0,
            failure: "oh no",
            witness: PhantomData
        };
        let change = |x: i32| x - 1;
        assert!(
            contramap_constraint(
                change,
                constraint
            ).run(1)
        )
    }
}
