trait Component {
    fn operation(&self);
    fn add(&mut self, component: impl Component + 'static);
    fn remove(&mut self, component: &impl Component);
    fn get_child(&self, index: usize) -> Option<&dyn Component>;
    fn parent(&self) -> Option<&dyn Component>;
}

struct Composite {
    parent: Option<&'static dyn Component>,
    children: Vec<&'static dyn Component>,
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

    fn add(&mut self, component: impl Component + 'static) {
        let erased: &'static dyn Component = Box::leak(Box::new(component));
        self.children.push(erased);
    }

    fn remove(&mut self, component: &impl Component) {
        self.children.retain(|&child| {
            !std::ptr::eq(child, component)
        });
    }

    fn get_child(&self, index: usize) -> Option<&dyn Component> {
        self.children.get(index).map(|c| *c)
    }

    fn parent(&self) -> Option<&dyn Component> {
        self.parent
    }
}

struct Leaf {
    parent: Option<&'static dyn Component>,
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

    fn add(&mut self, _component: impl Component + 'static) {
        panic!("Cannot add to a leaf");
    }

    fn remove(&mut self, _component: &impl Component) {
        panic!("Cannot remove from a leaf");
    }

    fn get_child(&self, _index: usize) -> Option<&dyn Component> {
        None
    }

    fn parent(&self) -> Option<&dyn Component> {
        self.parent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut composite = Composite::new();
        let mut leaf1 = Leaf::new();
        let mut leaf2 = Leaf::new();
        let mut leaf3 = Leaf::new();

        leaf1.parent = Some(&composite);
        leaf2.parent = Some(&composite);
        leaf3.parent = Some(&composite);

        composite.add(leaf1);
        composite.add(leaf2);
        composite.add(leaf3);

        composite.operation();
    }
}