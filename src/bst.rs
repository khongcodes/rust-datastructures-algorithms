//! Simple binary search tree implementation using Box references to Nodes.

// Following methods to be implemented
// [x] BinarySearchTree::new
// [x] BinarySearchTree::add_value
// [ ] BinarySearchTree::find_value - return true if present in tree
// [x] BinarySearchTree::remove_value
// [ ] BinarySearchTree::min -  return smallest value in tree
// [x] BinarySearchTree::print_inorder
// [x] BinarySearchTree::print_preorder
// [x] BinarySearchTree::print_postorder
// [ ] BinarySearchTree::height
//

use std::cmp::Ordering;
use crate::linked_list;


/// A binary search tree struct containing pointers to Node structs.
///
/// Values T in this tree must be comparable with < or > operators (or else this data structure is
/// nonsensical to use).
///
/// * `root`: An Option-wrapped reference to the root Node of the binary search tree.
///         This will be None if there are zero nodes in this tree.
pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>
}


/// A Node in a BinarySearchTree struct.
///
/// * `value`: Value held in this Node - must be comparable with < or > (implement Ord trait)
/// * `left_branch`: Option-wrapped reference to another Node, which should contain a Node with
///         lesser value. Initializes as None.
/// * `right_branch`: Option-wrapped reference to another Node, which should contain a Node with
///         greater value. Initializes as None.
pub struct Node<T: Ord> {
    value: T,
    left_branch: Option<Box<Node<T>>>,
    right_branch: Option<Box<Node<T>>>
}


/// Enum for traversal node order options on binary trees.
enum TreeTraversalOrders {
    Inorder, Preorder, Postorder
}


// Method implementation for BinarySearchTree struct
impl<T> BinarySearchTree<T> where T: Ord {
    
    /// Return a new, empty BinarySearchTree struct
    fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            root: None
        }
    }

    /// Add a Node to this BinarySearchTree struct.
    ///
    /// Accomplish this (if there is a root node) by beginning a recursive call to evaluate the new
    /// value against the current node's value, starting with the root member Node.
    ///
    /// It should be noted - if this value is evaluated as Ordering::Equal (== operator) to another Node's
    /// value in this tree, this value will be discarded without a Node being added.
    ///
    /// * `value`: The value to be added into the binary search tree.
    fn add_value(&mut self, value: T) {
        match &mut self.root {
            Some(boxed_node) => {
                boxed_node.add_value_as_child(value);
            },
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }
        }
    }

    /// Find input value in the BinarySearchTree (using Ordering::Equal (== operator)) and remove
    ///     it (and its enclosing Node).
    ///
    /// Node::remove_value_if_child is a recursive method that consumes the calling Node
    ///     struct and returns a new allocated Box to be assigned in place.
    ///
    /// * `value`: Value to be removed from the binary search tree.
    fn remove_value(&mut self, value: T) {
        if self.root.is_some() {
            self.root = self.root.take().unwrap().remove_value_if_child(&value);
        }
    }

    /// Create and return a vector containing references to the values held by Nodes in this
    /// BinarySearchTree struct.
    ///
    /// * `order`: A variant of TreeTraversalOrders enum that determines the orders of the value
    /// references in the returned vector
    fn collectpeek_traversal_values(&self, order: TreeTraversalOrders) -> Vec<&T> {
        let mut list = Vec::new();
        match order {
            TreeTraversalOrders::Inorder => { Node::collectpeek_inorder(&self.root, &mut list); },
            TreeTraversalOrders::Preorder => { Node::collectpeek_preorder(&self.root, &mut list); },
            TreeTraversalOrders::Postorder => { Node::collectpeek_postorder(&self.root, &mut list); }
        };
        list
    }

    /// Experimental version of previous method collectpeek_traversal_values_cratell that uses this
    /// crate's LinkedList struct instead of Vec. 
    ///
    /// Not fully implemented - LinkedList is less tested and therefore less reliable than standard library Vecs.
    ///
    /// A speed test should be created between this method and the previous method on
    ///     TreeTraversalOrders::Inorder.
    ///
    /// * `order`: A variant of TreeTraversalOrders enum that determines the orders of the value
    /// references in the returned vector
    fn collectpeek_traversal_values_cratell(&self, order: TreeTraversalOrders) -> linked_list::LinkedList<&T> {
        let mut list = linked_list::LinkedList::new();
        match order {
            TreeTraversalOrders::Inorder => { Node::collectpeek_inorder_cratell(&self.root, &mut list); },
            TreeTraversalOrders::Preorder => { todo!() },
            TreeTraversalOrders::Postorder => { todo!() }
        };
        list
    }
}


// Method implementations for Node struct in a BinarySearchTree struct.
impl<T> Node<T> where T: Ord {

    /// Return a new Node struct with the value T.
    ///
    /// * `value`: Value to be stored in the Node.
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left_branch: None,
            right_branch: None
        }
    }

    /// Return a reference to the value held in this Struct.
    fn peek_value(&self) -> &T {
        &self.value
    }

    /// Add a child Node to this Node with the input value.
    ///
    /// In binary search trees, if a new value to be added is less than a parent node's value, it
    /// should be added as a left-branch child of the parent node. If the new value is greater, it
    /// should be added as the right-branch child of the parent node. If the branch node already
    /// exists, the same operation should be done on the corresponding branch child node - recursively
    /// invoke this method on that branch child node with the same value.
    ///
    /// Needs a mutable self reference so it can assign to left_branch/right_branch members.
    ///
    /// * `value`: value to be held by the child Node to be added to this Node.
    fn add_value_as_child(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(boxed_node) = &mut self.left_branch {
                    boxed_node.add_value_as_child(value);
                } else {
                    self.left_branch = Some(Box::new(Node::new(value)));
                }
            },
            Ordering::Greater => {
                if let Some(boxed_node) = &mut self.right_branch {
                    boxed_node.add_value_as_child(value);
                } else  {
                    self.right_branch = Some(Box::new(Node::new(value)));
                }
            },
            Ordering::Equal => ()
        }
    }


    /// If this Node's value member matches the input value (== operator), remove self from tree by
    ///     calling remove_self_from_tree and replacing the current Node in place with its return value.
    ///     See remove_self_from_tree documentation on return value of this operation.
    ///
    /// It should be noted: the calling Node struct is consumed by this method if self's value members
    ///     matches the search value.
    /// 
    /// Otherwise, if input value member is less than Node's value member, compare it to
    ///     left_branch and assign left_branch member to be the return val of this function invoked
    ///     on the left_branch Node (if it is not None - if it is None, do nothing because this
    ///     value cannot be found in the tree)
    ///
    /// If this Node's value member is greater than Node's value member, compare it to right_branch
    ///     and assign right_branch member to be the return val of this function invoked on the
    ///     right_branch Node (if it is not None (see note above))
    ///     
    /// In both cases where this Node's value is not equal to the input value, return a new
    ///     allocation of this Node in the heap (Box) because we consume the original.
    ///
    /// * `value`: Value to be removed from the Node or its children branches.
    fn remove_value_if_child(mut self, value: &T) -> Option<Box<Node<T>>> {
        match value.cmp(&self.value) {
            Ordering::Less if self.left_branch.is_some() => {
                self.left_branch = self.left_branch.unwrap().remove_value_if_child(value);
            },
            Ordering::Greater if self.right_branch.is_some() => {
                self.right_branch = self.right_branch.unwrap().remove_value_if_child(value);
            },
            Ordering::Equal => { return self.remove_self_from_tree(); },
            _ => ()
        };
        Some(Box::new(self))
    }


    /// This self-consume method is run on a pointer to a Node - either from a parent or from the
    ///     BinarySearchTree struct - from the remove_value_if_child method, which assigns the
    ///     return value of this method to "paint over" the previous reference to this Node struct.
    ///
    /// This method fundamentally therefore controls the logic of what should take a deleted Node's
    ///     place in a BinarySearchTree struct, depending on its available children.
    ///
    /// If this Node has no children: replace self with None.
    ///
    /// If this Node has only right child or only left child: replace self with that child.
    ///
    /// If this Node has two children:
    /// 1. Find the smallest Node that is a child of this node's right child. From here on 
    ///     we will call the Node that we find NodeB, and self NodeA.
    ///     NodeA: { valueA }
    ///     NodeB: { valueB } (valueB < valueA)
    ///
    /// 2. Swap that node's value (valueB) with the value in self (valueA).
    ///     NodeA: { valueB }
    ///     NodeB: { valueA }
    ///
    /// 3. Recursively call down this child's right-child-branch (the branch of NodeA that 
    ///     terminates in NodeB) to delete the node with valueA, to make sure NodeB's parent
    ///     Node also sets its reference to NodeB as None.
    ///
    fn remove_self_from_tree(mut self) -> Option<Box<Node<T>>> {
        let left_child_exists = self.left_branch.is_some();
        let right_child_exists = self.right_branch.is_some();

        if !left_child_exists {
            if !right_child_exists {    // self has no children
                return None;
            } else {                    // self has only right child
                return self.right_branch;
            }
        }
        if !right_child_exists {        // self has only left child
            return self.left_branch;
        }

        let mutref_right_branch = self.right_branch.as_deref_mut().unwrap();
        // DELETE COMMENTS BELOW IF RETURNING &mut Node<T> works (instead of wrapping this in a Box)
        // Get a new Box pointer containing a mutable ref (from self) to the min child. This Box
        //  goes out of scope and is dumped at the end of this method, but that's okay because we
        //  only need it for the std::mem::swap call.
        let node_with_new_value = mutref_right_branch.find_minimum_child_below();

        std::mem::swap(&mut self.value, &mut node_with_new_value.value);
        

        // attempt copy value out of being a reference to self.right_branch... don't think this is
        // really possible.
        // let a = Box::new(node_with_new_value.value);

        // BECAUSE the input value is a reference, currently
        // I don't know if we can get out of just making T have the Copy trait bound.
        // self.right_branch = self.right_branch.unwrap().remove_value_if_child(a.as_ref());

        mutref_right_branch.drop_misaligned_child();
        // recursive downcall should happen here

        Some(Box::new(self))
    }


    /// Recursively return a new Box pointer (mutable) to the smallest child below self
    ///
    /// Helper method to remove_self_from_tree method.
    ///
    fn find_minimum_child_below(&mut self) -> &mut Node<T> {
        if self.left_branch.is_some() {
            self.find_minimum_child_below()
        } else {
            self
        }
    }


    fn drop_misaligned_child(&mut self) {
        if let Some(left_child) = &self.left_branch {
            if self.value < left_child.value {
                self.left_branch = None;
            } else {
                self.drop_misaligned_child();
            }
        }
    }


    // TODO: DELETE THIS WHOLE BLOCK IF UNNECESSARY
    // // allocate a pointer in heap (Box) to smallest child of this node, return raw pointer
    // fn find_mut_ref_minimum_child_below(mut self, ) -> (Node<T>, *mut Node<T>) {
    //     if let Some(smaller_node) = self.left_branch {
    //         smaller_node.find_mut_ref_minimum_child_below()
    //     } else {
    //         (self, )
    //         Box::into_raw(Box::new(self))
    //     }
    // }
    //
    //
    // fn find_min_value_below(&self) -> &T {
    //     
    // }
    //
    //
    // fn swap_value(other_node: *mut &Node<T>, this_node: Node<T>) -> Option<Box<Node<T>>> {
    //    Some(Box::new(this_node)) 
    // }
    //
    // fn swap_with_min_child_below(&mut self) {
    //     let node_with_new_value = self.right_branch.as_deref_mut().unwrap();
    // }


    /// Assign to a Vec (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using inorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the Vec where Node value references should be added.
    fn collectpeek_inorder<'a>(
        opt_node: &'a Option<Box<Node<T>>>,
        list: &mut Vec<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            Node::collectpeek_inorder(&boxed_node.left_branch, list);
            list.push(&boxed_node.value);
            Node::collectpeek_inorder(&boxed_node.right_branch, list);
        }
    }

    /// Assign to a LinkedList (from this crate) (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using inorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the LinkedList where Node value references should be added.
    fn collectpeek_inorder_cratell<'a>(
        opt_node: &'a Option<Box<Node<T>>>, 
        list: &mut linked_list::LinkedList<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            Node::collectpeek_inorder_cratell(&boxed_node.left_branch, list);
            list.add_value(&boxed_node.value);
            Node::collectpeek_inorder_cratell(&boxed_node.right_branch, list);
        }
    }

    /// Assign to a Vec (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using preorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the Vec where Node value references should be added.
    fn collectpeek_preorder<'a>(
        opt_node: &'a Option<Box<Node<T>>>, 
        list: &mut Vec<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            list.push(&boxed_node.value);
            Node::collectpeek_preorder(&boxed_node.left_branch, list);
            Node::collectpeek_preorder(&boxed_node.right_branch, list);
        }
    }

    /// Assign to a Vec (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using postorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the Vec where Node value references should be added.
    fn collectpeek_postorder<'a>(
        opt_node: &'a Option<Box<Node<T>>>, 
        list: &mut Vec<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            Node::collectpeek_postorder(&boxed_node.left_branch, list);
            Node::collectpeek_postorder(&boxed_node.right_branch, list);
            list.push(&boxed_node.value);
        }
    }
}


////////////////////////////////////////////////////////////////////////////
//  TESTS

fn setup_bst() -> BinarySearchTree<u32> {
    let mut bst: BinarySearchTree<u32> = BinarySearchTree::new();
    bst.add_value(4);
    bst.add_value(2);
    bst.add_value(6);
    bst.add_value(1);
    bst.add_value(3);
    bst.add_value(5);
    bst
    //      4
    //     / \
    //   2    6
    //  /\   /
    // 1 3  5
}

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn bst_can_be_created_and_added_to()  {
        let bst = setup_bst();
        assert!(&bst.root.is_some());
        assert_eq!(bst.root.unwrap().value, 4);
    }

    // #[test]
    fn bst_height_can_be_evaluated() {
        let bst = setup_bst();
    }

    #[test]
    fn bst_can_delete_nodes() {
        let mut bst = setup_bst();
        bst.remove_value(1);
        bst.remove_value(2);
        let mut list_iter = bst.collectpeek_traversal_values(TreeTraversalOrders::Inorder).into_iter();
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&4));
        assert_eq!(list_iter.next(), Some(&5));
        assert_eq!(list_iter.next(), Some(&6));
    }

    #[test]
    fn bst_can_be_traversed_inorder() {
        let bst = setup_bst();
        let list = bst.collectpeek_traversal_values(TreeTraversalOrders::Inorder);
        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&4));
        assert_eq!(list_iter.next(), Some(&5));
        assert_eq!(list_iter.next(), Some(&6));
    }

    #[test]
    fn bst_can_be_traversed_inorder_with_cratell() {
        let bst = setup_bst();
        let mut list = bst.collectpeek_traversal_values_cratell(TreeTraversalOrders::Inorder);
        assert_eq!(list.dequeue_value(), Some(&1));
        assert_eq!(list.dequeue_value(), Some(&2));
        assert_eq!(list.dequeue_value(), Some(&3));
        assert_eq!(list.dequeue_value(), Some(&4));
        assert_eq!(list.dequeue_value(), Some(&5));
        assert_eq!(list.dequeue_value(), Some(&6));
    }

    #[test]
    fn bst_can_be_traversed_preorder() {
        let bst = setup_bst();
        let list = bst.collectpeek_traversal_values(TreeTraversalOrders::Preorder);
        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(&4));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&6));
        assert_eq!(list_iter.next(), Some(&5));
    }

    #[test]
    fn bst_can_be_traversed_postorder() {
        let bst = setup_bst();
        let list = bst.collectpeek_traversal_values(TreeTraversalOrders::Postorder);
        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&5));
        assert_eq!(list_iter.next(), Some(&6));
        assert_eq!(list_iter.next(), Some(&4));
    }
}
