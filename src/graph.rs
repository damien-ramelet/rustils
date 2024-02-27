use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[allow(dead_code)]
struct Node {
    name: String,
    edges: RefCell<Vec<Rc<Edge>>>,
}

#[allow(dead_code)]
struct Edge {
    weight: i32,
    nodes: RefCell<Vec<Weak<Node>>>,
}

#[allow(dead_code)]
struct Graph {
    nodes: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::graph::{Node, Edge};
    
    #[test]
    fn we_can_build_simple_weighted_graph() {
        let edge = Rc::new(Edge {
            weight: 10,
            nodes: RefCell::new(vec![]),
        });
        let first_node = Rc::new(Node {
            name: String::from("First node"),
            edges: RefCell::new(vec![Rc::clone(&edge)]),
        });
        let second_node = Rc::new(Node {
            name: String::from("Second node"),
            edges: RefCell::new(vec![Rc::clone(&edge)]),
        });
        
        edge.nodes.borrow_mut().push(Rc::downgrade(&first_node));
        edge.nodes.borrow_mut().push(Rc::downgrade(&second_node));
        
        assert_eq!(Rc::strong_count(&edge), 3);
        assert!(Rc::strong_count(&first_node) == Rc::strong_count(&second_node) && Rc::strong_count(&second_node) == 1);
        assert!(first_node.edges.borrow().len() == 1 && second_node.edges.borrow().len() == 1);
        assert!(edge.nodes.borrow().len() == 2);
    }
}
