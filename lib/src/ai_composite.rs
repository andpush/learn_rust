trait Component {
    fn operation(&self);
    fn add(&mut self, component: Box<dyn Component>);
    fn remove(&mut self, component: Box<dyn Component>);
    fn get_child(&self, index: usize) -> Option<&Box<dyn Component>>;
    fn parent(&self) -> Option<&Box<dyn Component>>;
}

struct Composite {
    parent: Option<Box<dyn Component>>,
    children: Vec<Box<dyn Component>>,
}

impl Composite {
    fn new() -> Self {
        Composite {
            parent: None,
            children: vec![],
        }
    }
}

impl Component for Composite {
    fn operation(&self) {
        println!("Composite operation");
        for child in &self.children {
            child.operation();
        }
    }

    fn add(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }

    fn remove(&mut self, component: Box<dyn Component>) {
        self.children.retain(|child| {
            !std::ptr::eq(child.as_ref(), component.as_ref())
        });
    }

    fn get_child(&self, index: usize) -> Option<&Box<dyn Component>> {
        self.children.get(index)
    }

    fn parent(&self) -> Option<&Box<dyn Component>> {
        self.parent.as_ref()
    }
}

struct Leaf {
    parent: Option<Box<dyn Component>>,
}

impl Leaf {
    fn new() -> Self {
        Leaf { parent: None }
    }
}

impl Component for Leaf {
    fn operation(&self) {
        println!("Leaf operation");
    }

    fn add(&mut self, _component: Box<dyn Component>) {
        panic!("Cannot add to a leaf");
    }

    fn remove(&mut self, _component: Box<dyn Component>) {
        panic!("Cannot remove from a leaf");
    }

    fn get_child(&self, _index: usize) -> Option<&Box<dyn Component>> {
        None
    }

    fn parent(&self) -> Option<&Box<dyn Component>> {
        self.parent.as_ref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut composite = Box::new(Composite::new());
        let mut leaf1 = Box::new(Leaf::new());
        let mut leaf2 = Box::new(Leaf::new());
        let mut leaf3 = Box::new(Leaf::new());

        leaf1.parent = Some(composite.clone());
        leaf2.parent = Some(composite.clone());
        leaf3.parent = Some(composite.clone());

        composite.add(leaf1);
        composite.add(leaf2);
        composite.add(leaf3);

        composite.operation();
    }

}
