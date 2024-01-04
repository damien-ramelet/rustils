use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Vec<Weak<Node>>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
struct Tree {
    nodes: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use crate::tree::{Node, Tree};

    #[test]
    fn tree_can_have_nodes() {
        let node = Rc::new(Node {
            value: 0,
            parent: RefCell::new(vec![]),
            children: RefCell::new(vec![]),
        });
        let tree = Tree {
            nodes: RefCell::new(vec![Rc::clone(&node)]),
        };
        
        assert_eq!(tree.nodes.borrow_mut().len(), 1);
        assert_eq!(Rc::strong_count(&node), 2);
    }
    
    #[test]
    fn parent_can_own_children() {
        let child = Rc::new(Node {
            value: 0,
            parent: RefCell::new(vec![]),
            children: RefCell::new(vec![]),
        });
        
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(vec![]),
            children: RefCell::new(vec![Rc::clone(&child)]),
        });
        
        assert_eq!(parent.children.borrow().len(), 1);
        assert_eq!(Rc::strong_count(&child), 2);
        assert_eq!(Rc::strong_count(&parent), 1);
    }
    
    #[test]
    fn children_can_reference_parent() {
        let child = Rc::new(Node {
            value: 0,
            parent: RefCell::new(vec![]),
            children: RefCell::new(vec![]),
        });
        
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(vec![]),
            children: RefCell::new(vec![Rc::clone(&child)]),
        });
        
        child.parent.borrow_mut().push(Rc::downgrade(&parent));
        
        assert_eq!(child.parent.borrow().len(), 1);
        assert_eq!(Rc::strong_count(&child), 2);
        assert_eq!(Rc::strong_count(&parent), 1);
    }

    #[test]
    fn we_dont_create_cyclic_references() {
        let children = Rc::new(Node {
            value: 0,
            parent: RefCell::new(vec![]),
            children: RefCell::new(vec![]),
        });
        
        let parent = Rc::new(Node {
            value: 0,
            parent: RefCell::new(vec![]),
            children: RefCell::new(vec![Rc::clone(&children)]),
        });
        
        children.parent.borrow_mut().push(Rc::downgrade(&parent));
        
        let parent_ref_count = Rc::strong_count(&parent);
        let children_ref_count = Rc::strong_count(&children);
        
        assert_eq!(parent_ref_count, 1);
        assert_eq!(children_ref_count, 2);
        assert_eq!(Rc::weak_count(&parent), 1);
    }
}
