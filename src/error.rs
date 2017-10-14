pub mod error {
    pub enum Error {
        Pure {value: String}
        Not {value: Error}
        And {left: Error, right: Error}
        Or {left: Error, right: Error}

        fn of (x: String) -> Error {
            Pure {value: x}
        }
    }

    pub fn cata (x: Error) -> String {
        match (x) {
            Pure (x) => x
            Not (negated) => format!("Not ({})", cata (negated))

        }
    }
}
