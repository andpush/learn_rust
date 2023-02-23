
// Equality is deep in Rust
fn main() {
    let v1 = vec![2, 4, 6].clone();
    let v2 = vec![2, 4, 6];
    assert_eq!(v1, v2);

    let s1 = Box::new("Hey there!");
    let s2 = Box::new("Hey there!");
    assert_eq!(s1, s2);

    let p1 = Point{x:12, y:24};
    let p2 = Point{x:12, y:24};
    assert_eq!(p1, p2);

}

#[derive(PartialEq, Debug)]
struct Point {
    x:i32,
    y:i32,
}