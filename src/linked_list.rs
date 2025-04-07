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


    // Method implementtations for LinkedList struct
    impl<T> LinkedList<T> {


        /// Return a new, empty LinkedList struct with empty Weak references in head and tail
        /// members and an empty vector for nodes member
        pub fn new_ll() -> LinkedList<T> {
            return LinkedList {
                nodes: Vec::new(),
                head: Weak::new(),  // calling upgrade on this returns None
                tail: Weak::new()
            };
        }


        /// Add a Node containing value T to the end of the Linked List (make the new Node the
        /// next member of the current tail Node)
        ///
        /// * `value`: T (matching the LinkedList's generic type parameter) to be stored in a new
        ///         Node in the LinkedList
        pub fn add_value(&mut self, value: T) {
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


        pub fn peek_head_value(&self) -> Option<&T> {
            // check if the head Weak reference resolves; if it does we can call the unsafe
            //      pointers to deref it and get ta reference to its value without consuming it
            return match self.head.upgrade() {
                Some(_) => Some( unsafe { &(*(*self.head.as_ptr()).as_ptr()).value } ),
                None => None
            };
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

