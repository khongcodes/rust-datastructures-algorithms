//! Simple linked list implementation that works using std::rc::{ Rc, Weak }, and interior
//!     mutability with RefCell. Non-threadsafe (can be re-implemented with Arc and Mutex)

pub mod linked_list {

    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    /// Node<T> struct:
    ///
    /// Node in a LinkedList struct. Contains a value and a Weak reference to the following Node in
    /// the linked list.
    ///
    /// * `value`: T must be of type that matches the LinkedList struct that this Node can be placed in
    /// * `next`: Weak referencce, with interior mutability, to the next Node
    ///
    /// The effect of using Weak references here makes it so that if Nodes reference each other, such
    /// cycles cannot lead to memory leaks due to inaccessible standing references if pointers to these
    /// values are ever reassigned.
    ///
    /// The effect of using interior mutability (via RefCell) makes it so we can assign a new next Node
    /// at the point when a new Node is added to the LinkedList struct.
    pub struct Node<T> {
        pub value: T,
        pub next: Weak<RefCell<Node<T>>>
    }


    /// A linked list struct containing "pointers" to Node structs.
    ///
    /// Head and tail members contain weak references to Node structs wrapped in
    /// RefCell so their next members can be mutated, if the Node structs have not yet been dropped,
    /// but will not count as references that will prevent the Nodes from being dropped.
    ///
    /// * `nodes`: A vector containing owned-Rcs containing the Nodes of the LinkedList
    /// * `head`: A weak reference to a Node, (will not count against Node being dropped, must be
    ///             resolved to an Option<RefCell<Node<T>>> in order to be accecssed with upgrade()
    /// * `tail`: Same as head
    pub struct LinkedList<T> {
        pub nodes: Vec<Rc<RefCell<Node<T>>>>,      // This member is in place to holds ownership of node data
        pub head: Weak<RefCell<Node<T>>>,
        pub tail: Weak<RefCell<Node<T>>>
    }


    impl<T> LinkedList<T> {

        
        fn new_ll() -> LinkedList<T> {
            return LinkedList {
                nodes: Vec::new(),
                head: Weak::new(),  // calling upgrade on this returns None
                tail: Weak::new()
            };
        }


        fn add_value(&mut self, value: T) {
            let new_node: Rc<RefCell<Node<T>>>;

            new_node = new_boxed_node(value);

            // upgrade turns Weak<T> into Option<Rc<T>>
            match self.tail.clone().upgrade() {
                Some(node_ref) => { 
                    node_ref.borrow_mut().next = Rc::downgrade(&new_node); 
                },
                None => { 
                    self.head = Rc::downgrade(&new_node); 
                },
            }
            self.tail = Rc::downgrade(&new_node);
            self.nodes.push(new_node);
        }
    }

    // fn new_ll<T>() -> LinkedList<T> {
    //     return LinkedList {
    //         nodes: Vec::new(),
    //         head: Weak::new(),  // calling upgrade on this returns None
    //         tail: Weak::new()
    //     };
    // }

    fn new_node<T>(value: T) -> Node<T> {
        return Node {
            value,
            next: Weak::new()
        };
    }

    fn new_boxed_node<T>(value: T) -> Rc<RefCell<Node<T>>> {
        return Rc::new(RefCell::new(new_node(value)));
    }
}

