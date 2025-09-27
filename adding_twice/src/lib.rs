pub fn add_curry(a: i32) -> impl Fn(i32) -> i32 {
    move |b| b + a
}

pub fn twice<T>(F: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |a| F(F(a))
}