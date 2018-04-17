#![feature(conservative_impl_trait)]
#[allow(unused_imports)]
#[macro_use]
extern crate quickcheck;

mod boolean;

#[cfg(test)]
mod tests {
    use boolean::expression::not;
    use boolean::expression::cata;
    use boolean::expression::of;
    use boolean::expression::or;
    use boolean::expression::and;
    use boolean::expression::context;

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
}
