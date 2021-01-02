use std::cell::RefCell;
use std::rc::Rc;
use crate::structures::rc_node::{NodeDir, Node, Tree};

/// Public trait that houses methods that are used with Tree classes
#[derive(Debug)]
pub struct BST<T>  {
    root: NodeDir<T>,
    pub count: u32
}
impl<T> Tree<T> for BST<T>  {
    /// Function to set the left-most branch node to another NodeDir
    fn set_left(&mut self, node: NodeDir<T>){
        self.root.as_ref().unwrap()
            .borrow_mut().left = node
    }
    /// Function to set the right-most branch node to another NodeDir
    fn set_right(&mut self, node: NodeDir<T>){
        self.root.as_ref().unwrap()
            .borrow_mut().right = node
    }

    fn get_left(&self) -> NodeDir<T>{
        self.root.as_ref().unwrap()
            .borrow_mut().left.clone()
    }

    fn get_right(&self) -> NodeDir<T>{
        self.root.as_ref().unwrap()
            .borrow_mut().right.clone()
    }

    fn is_right(&self) -> bool {
        self.root.as_ref().unwrap()
            .borrow_mut().right.is_some()
    }

    fn is_left(&self) -> bool {
        self.root.as_ref().unwrap()
            .borrow_mut().left.is_some()
    }
}
impl<T> BST<T>  {
    /// Initialize BST with nothing
    /// the next value added is automatically the root
    pub fn new()->Self{
        BST {
            root: None,
            count: 0
        }
    }
    /// Checks if the BST<T> is empty
    pub fn is_empty(&self)-> bool{self.count == 0}
    /// Creates another node with a key data pair
    fn new_node_dir(&self, key:u32, data: T) -> NodeDir<T>{
        Some(Rc::new(RefCell::new(Node::new(key,data))))
    }
    /// Adds a node with kv pair to the BST
    pub fn add(&mut self, key:u32, data: T){
        if self.is_empty(){
            self.root = self.new_node_dir(key, data);
            self.count = 1;
            return;
        }
        let kv_pair = (key, data);
        let curr = Rc::clone(self.root
            .as_ref().unwrap());
        self.count += 1;
        self.recursive_add(kv_pair, curr);
    }
    ///Helper method for add
    fn recursive_add(&mut self, kv_p:(u32, T), node: Rc<RefCell<Node<T>>>){
        let mut inner_value:
            std::cell::RefMut<Node<T>> = (*node).borrow_mut();

        if &kv_p.0 <= &inner_value.key {
            if let None = inner_value.get_left(){
                inner_value.set_left(self.new_node_dir(kv_p.0,kv_p.1))
            }else {
                let left = Rc::clone(inner_value.left
                    .as_ref().unwrap());
                self.recursive_add(kv_p,left);
            }
            return;
        }
        if &kv_p.0 > &inner_value.key{
            if let None = inner_value.right {
                inner_value.set_right(self.new_node_dir(kv_p.0,kv_p.1))
            }else{
                let right = Rc::clone(inner_value.left
                    .as_ref().unwrap());
                self.recursive_add(kv_p,right);
            }
            return;
        }
    }
    /// Recurse through the right most node ( if there is )
    /// to find the node with the largest node
    fn _max_(&self, node: &NodeDir<T>) ->NodeDir<T>{
        let mut last = node.clone();
        let mut next = node.clone();
        while let Some(node) = next{
            next = node.borrow().right.clone();
            last = Some(node)
        }
        last
    }
    /// Recurse through the left most node ( if there is )
    /// to find the node with the lowest node
    fn _min_(&self, node:&NodeDir<T>)-> NodeDir<T>{
        let mut last = node.clone();
        let mut next = node.clone();
        while let Some(node) = next{
            next = node.borrow().left.clone();
            last = Some(node)
        }
        last
    }
    pub fn min(&self)-> NodeDir<T>{
        return if let None = self.root{
            None
        }
        else {
            self._min_(&self.root)
        }
    }
    pub fn max(&self)-> NodeDir<T>{
        return if let None = self.root{
            None
        }
        else {
            self._max_(&self.root)
        }
    }
    pub fn find(&self, key:u32)->NodeDir<T>{
        return if let None = self.root{
            None
        }
        else {
            self._find(key,&self.root)
        }
    }
    fn _find(&self,key: u32, node:&NodeDir<T>)-> NodeDir<T>{
        let mut last = node.clone();
        let mut next = node.clone();
        while let Some(node) = next{
            next = {
                let node_key = node.borrow().key;
                if &key < &node_key{
                    node.borrow().left.clone()
                }else {
                    node.borrow().right.clone()
                }
            };
            last = Some(node)
        }
        last
    }
}
pub fn sort<T>(arr: Vec<T>){
    unimplemented!()
}
// let node = if let Some(node) = node {
//     node
// } else {
//     return None
// };
// self._min_(&node.borrow().left).or(node.borrow().left.clone())