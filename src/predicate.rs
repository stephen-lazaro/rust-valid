trait Profunctor <A, B> {
    fn dimap <C, D> (f: C -> A, g: B -> D)
}

trait Predicate <T, Repr, Error> {
    fn applyUnsafe (x: Repr) -> Result <T, Error>

    fn apply (x: Repr) -> Result <T, Error> {
      
    }
}
