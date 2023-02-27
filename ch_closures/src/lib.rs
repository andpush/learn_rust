fn hi(n:i32){
    println!("Hi {} ", n);
}

// repeatedly executes closure passed as first argument n times, passed as second argument
fn rep(foo:impl Fn(i32), m:i32) {
    for i in 0..m {
        foo(i);
    }
}

fn rep2<T: Fn(i32)> (foo: T, m:i32) {
    for i in 0..m {
        foo(i);
    }
}

fn rep3(foo:&dyn Fn(i32), m:i32) {
    for i in 0..m {
        foo(i);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let title = "my friend";
        let f = |i: i32| println!("Hi {}: {}", title, i);
        f(999);
        rep(f, 2);
        rep(hi, 3);
    }
}
