//! Simple linked list implementation that works using std::rc::{ Rc, Weak }, and interior
//!     mutability with RefCell. Non-threadsafe (can be re-implemented with Arc and Mutex)

pub mod linked_list {

    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    /// Node in a LinkedList struct. Contains a value and an Option-wrapped Rc reference to the following Node in
    /// the linked list (with interior mutability)
    ///
    /// * `value`: T must be of type that matches the LinkedList struct that this Node can be placed in
    /// * `next`: Option holding an Rc to the next node (should be the only reference but a Weak
    ///     upgradable reference for any node should be allowed to exist for the purposes of updating
    ///     the linked list tail member)
    ///
    /// The effect of using Rc references here makes it so that any Node can be mutated by
    /// accessing it from the preceded Node (a valid mutation would be assigning a subsequent new
    /// Node in the LinkedList), and allowing a Weak reference to be made for the tail member in
    /// the LinkedList.
    ///
    /// The effect of using interior mutability (via RefCell) makes it so we can assign a new next Node
    /// at the point when a new Node is added to the LinkedList struct.
    ///
    /// There is a danger in not using weak references for this Next member that two Nodes can
    /// point to each other and cause circular reference. For that reason, Node next values should
    /// only be updated using our defined methods with implementation.
    pub struct Node<T> {
        pub value: T,
        next: Option<Rc<RefCell<Node<T>>>>
    }


    /// A linked list struct containing "pointers" to Node structs.
    ///
    /// Head and tail members contain references to Node structs wrapped inside.
    /// RefCell so their next members can be mutated, if the Node structs have not yet been dropped,
    /// but will not count as references that will prevent the Nodes from being dropped.
    ///
    /// * `head`: An Option-wrapped reference to a Node that allows for a weak reference to the same
    ///     Node to be created. 
    /// * `tail`: Weak reference to a Node, (will not count against Node being dropped, must be
    ///     resolved to an Option<RefCell<Node<T>>> in order to be accecssed with upgrade()

    pub struct LinkedList<T> {
        pub head: Option<Rc<RefCell<Node<T>>>>,
        pub tail: Weak<RefCell<Node<T>>>
    }


    // Method implementtations for LinkedList struct
    impl<T> LinkedList<T> {

        /// Return a new, empty LinkedList struct
        pub fn new() -> LinkedList<T> {
            return LinkedList {
                head: None,
                tail: Weak::new()   // calling upgrade on this returns None; empty allocation
            };
        }

        /// Add a Node containing value T to the end of the Linked List (make the new Node the
        /// next member of the current tail Node)
        ///
        /// * `value`: T (matching the LinkedList's generic type parameter) to be stored in a new
        ///         Node in the LinkedList
        pub fn add_value(&mut self, value: T) {
            let new_node: Rc<RefCell<Node<T>>>;
            new_node = Node::new_ref_wrapped(value);
            
            // use clone() to not consume (thus invalidating) the existing tail member
            // upgrade turns Weak<T> into Option<Rc<T>>
            match self.tail.clone().upgrade() {
                Some(node_ref) => { 
                    self.tail = Rc::downgrade(&new_node);
                    node_ref.borrow_mut().next = Some(new_node); 
                },
                None => {
                    self.tail = Rc::downgrade(&new_node);
                    self.head = Some(new_node); 
                },
            }
        }
        
        /// Get a reference to the value in the head member (if the head member is not None)
        pub fn peek_head_value(&self) -> Option<&T> {
            // Option<T>.clone() -> Option<&T> (an Option<Rc<RefCell<Node<T>>>> we can consume as
            //      it's a clone of self.head)
            // Option<T>.map() - returns None or Some(T mapped)
            return self.head.clone().map(|rc| Node::peek_val(Rc::clone(&rc)) );
        }
    }

    impl<T> Node<T> {
        
        /// Return a new Node with the value T
        fn new(value: T) -> Node<T> {
            return Node {
                value,
                next: None
            };
        }

        /// Same as new but returns the value wrapped in Rc<RefCell>> to enable multiple Weak
        /// references to be made with interior mutability
        fn new_ref_wrapped(value: T) -> Rc<RefCell<Node<T>>> {
            return Rc::new(RefCell::new(Node::new(value)));
        }

        // uses unsafe code but only by dereferencing a raw pointer to a struct in order to create a
        //      new ref to one of the struct's fields... lifetime of ref is still same as struct's.
        fn peek_val<'a>(node_ref: Rc<RefCell<Node<T>>>) -> &'a T {
            return unsafe { &(*node_ref.as_ptr()).value };
        }

    }


    // TODO: create validation so that when a node is assigned a next Node, it checks how many
    // strong ref count
    


}

