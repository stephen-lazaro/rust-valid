pub fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(B) -> C,
        G: Fn(A) -> B,
{
    move |x| f(g(x))
}
