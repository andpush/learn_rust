mod ai_composite;
mod ai_composite2;

use std::fmt::{Display, Formatter, Pointer};

#[derive(PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn dist(&self, p: Point) -> f64 {
        let dx = p.x - self.x;
        let dy = p.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}

trait Dist {
    type T; // associated Type
    fn dista(&self, another: Self::T)->f64;
}


impl Dist for Point {
    type T = Point;
    fn dista(&self, p:Point) -> f64 {self.dist(p)}
}

// no inheritance of structure in Rust, but we have structural composition, trait inheritance and generics
#[derive(Clone)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Dist for Point3d {
    type T = Point3d;
    fn dista(&self, p:Point3d) -> f64 {
        let dx = p.x - self.x;
        let dy = p.y - self.y;
        let dz = p.z - self.z;
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

// Composite struct
struct Comp<'a> {
    name: String,
    parent: Option<&'a Self>,
    children: Vec<&'a Self>,
}

impl<'a> Comp<'a> {
    fn new(name: String) -> Comp<'a> {
        Comp{name, parent: None, children: Vec::new()}
    }

    fn add_child (&mut self, name: String) -> Comp<'a> {
        let mut c = Comp::new(name);
        c.parent = Some(&self);
        // &self.children.push(&c);
        c
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equals() {
        let b1 = "String";
        let b2 = "String".to_owned();
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_point_dist() {
        assert_eq!(Point{x:1.0, y:2.0}.dist(Point{x:4.0, y:6.0}), 5.0);
    }

    #[test]
    fn test_point_dista() {
        assert_eq!(Point{x:1.0, y:2.0}.dista(Point{x:4.0, y:6.0}), 5.0);
    }

    #[test]
    fn test_point3d_same_point() {
        let p = Point3d{ x:2.0, y:3.0, z:5.0};
        assert_eq!(p.dista(p.clone()), 0.0);
    }

    #[test]
    fn test_point3d() {
        let p = Point3d{ x:3.0, y:4.0, z:5.0};
        assert_eq!(p.dista(Point3d{x:0.0, y:0.0, z:0.0}).round(), 7.0);
    }

    #[test]
    fn test_component() {
        let mut root = Comp::new("A".to_string());
        let a1 = root.add_child("A1".to_string());
        let mut a2 = root.add_child("A2".to_string());
        let a3 = root.add_child("A3".to_string());
        let a21 = a2.add_child("A21".to_string());
        let a22 = a2.add_child("A22".to_string());
        println!("$root");
    }
}
