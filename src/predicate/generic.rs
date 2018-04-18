pub fn compose<'a, A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(B) -> C + 'a,
        G: Fn(A) -> B + 'a,
{
    move |x| f(g(x))
}
