//! Simple linked list implementation that works using std::rc::{ Rc, Weak }, and interior
//!     mutability with RefCell. Non-threadsafe (can be re-implemented with Arc and Mutex)

use std::rc::{Rc, Weak};
use std::cell::RefCell;

///
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
// #[derive(PartialEq, Eq)]
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


// Method implementations for LinkedList struct
impl<T> LinkedList<T> {

    /// Return a new, empty LinkedList struct
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: Weak::new()   // calling upgrade on this returns None; empty allocation
        }
    }

    /// Add a Node containing value T to the end of the Linked List (make the new Node the
    /// next member of the current tail Node)
    ///
    /// * `value`: T (matching the LinkedList's generic type parameter) to be stored in a new
    ///         Node in the LinkedList
    pub fn add_value(&mut self, value: T) {
        let new_node: Rc<RefCell<Node<T>>> = Node::new_ref_wrapped(value);
        
        // use clone() to not consume (thus invalidating) the existing tail member
        // upgrade turns Weak<T> into Option<Rc<T>>
        match self.tail.clone().upgrade() {
            Some(node_ref) => { 
                self.tail = Rc::downgrade(&new_node);
                node_ref.borrow_mut().assign_next(Some(new_node)); 
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
        self.head.clone().map(|rc| Node::peek_val(Rc::clone(&rc)) )
    }

    /// Removes the current head from the LinkedList and returns it.
    /// The current head's next member becomes tthe new head.
    pub fn dequeue(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        match self.head.take() {
            Some(node_rc) => {
                let new_head: Option<Rc<RefCell<Node<T>>>> = node_rc.borrow_mut().get_next();
                self.head = new_head;
                Some(node_rc)
            },
            None => None
        }
    }
}



impl<T> Node<T> {
    
    /// Return a new Node with the value T
    ///
    /// * `value`: Value to be stored in the Node.
    fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None
        }
    }

    /// Same as new but returns the value wrapped in Rc<RefCell>> to enable multiple Weak
    /// references to be made with interior mutability.
    ///
    /// * `value`: Value to be stored in the Node.
    fn new_ref_wrapped(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node::new(value)))
    }

    /// Return an immutable reference to the value held in this Node.
    /// Uses unsafe code but only by dereferencing a raw pointer to a struct in order to create a
    ///     new ref to one of the struct's fields... lifetime of ref is still same as struct's.
    ///
    /// * `node_ref`: Rc<RefCell<Node>> to peek into - can be created from Rc::clone.
    ///         We use a strong reference here to assert that the Rc can't be None.
    ///         
    ///     WARNING: Funny story: you should call Rc::cloneto create node_ref so that it is not 
    ///     the only reference to the Rc value passed in here, or else the returned value may be
    ///     invalidated/dropped when the input node_ref Rc drops at the end of this function
    pub fn peek_val<'a>(node_ref: Rc<RefCell<Node<T>>>) -> &'a T {
        unsafe { &(*node_ref.as_ptr()).value }
    }

    /// Assign this Node's next member as the input Rc<RefCell<Node>>.
    ///
    /// * `node_rc`: This should be an Rc<RefCell<Node>>; strong reference to a Node Rc
    fn assign_next(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        match node {
            Some(node_rc) => {
                // disallow immediate circular reference
                if node_rc.as_ptr() == self {
                    panic!("Can't assign next to input Node - input Node's next is this Node; circular reference!");
                }
                self.next = Some(node_rc);
            },
            None => { 
                self.next = None; 
            }
        }
    }

    /// Assign this Node's next member to be the input Rc<RefCell<Node>> - but first take the input
    ///     Rc<RefCell<Node>> and assign its next member to be this Node's current next member
    ///
    /// * `node_rc`: 
    pub fn splice_in_next(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        match node {
            Some(node_rc) => {
                // clone() is required to get an owned copy of self.next from out of this
                // mut ref self
                node_rc.borrow_mut().assign_next(self.next.clone());
                self.assign_next(Some(node_rc));
            },
            None => {   // discard remaining nodes
                self.assign_next(None);
            }
        }
    }

    /// A function that solely invalidates the Node's next member and returns it.
    fn get_next(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        self.next.take()
    }

}


////////////////////////////////////////////////////////////////////////////
//  TESTS

fn setup_linked_list() -> LinkedList<u32>  {
    let mut ll: LinkedList<u32> = LinkedList::new();
    ll.add_value(2);
    ll.add_value(4);
    ll
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_works() {
        let basic_ll = setup_linked_list();
        let result: Option<&u32> = basic_ll.peek_head_value();
        assert!(result.is_some_and(|x| *x == 2));
    }

    #[test]
    fn node_peek_val_works() {
        let mut basic_ll = setup_linked_list();
        let a = basic_ll.dequeue().unwrap();
        let b = Node::peek_val(a.clone());
        assert_eq!(b, &2);
        assert_eq!(std::rc::Rc::strong_count(&a), 1);
    }

    #[test]
    fn dequeue_works() {
        let mut basic_ll = setup_linked_list();
        let result_1 = basic_ll.dequeue();
        let result_2: Option<&u32> = basic_ll.peek_head_value();

        assert!(result_1.is_some_and(|x| x.borrow().value == 2));
        assert!(result_2.is_some_and(|x| *x == 4));
    }

    #[test]
    fn splice_works() {
        let mut basic_ll = setup_linked_list();
        let a = basic_ll.head.clone().unwrap();
        a.borrow_mut().splice_in_next(Some(Node::new_ref_wrapped(6)));
        basic_ll.dequeue();

        assert!(basic_ll.peek_head_value().is_some_and(|x| *x == 6));
    }
}
