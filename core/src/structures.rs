use std::{
    cell::RefCell,cmp::Ordering, collections::VecDeque,
    env,io::{self,Write},path::Path,rc::Rc
};
use std::borrow::{BorrowMut, Borrow};

///Optional reference to another Node of generic T type
pub type NodeDir<T> = Option<Rc<RefCell<Node<T>>>>;

///Node with generic T type.
///Key is separated from data in case generic doesnt have std::cmp
///
/// right and left nodes connecting to it are optional references
pub struct Node<T>{
    pub key: u32,
    data: T,
    pub left: NodeDir<T>,
    pub right: NodeDir<T>
}
impl<T> Node<T>{
    pub fn new(key: u32, data: T)-> Self{
        Node{
            key,
            data,
            left: None,
            right: None
        }
    }
}
/// Public trait that houses methods that are used with Tree classes
pub trait Tree<T>{
    /// Sets the left node from a node
    fn set_left(&mut self, node: NodeDir<T>);
    /// Sets a right node from a node
    fn set_right(&mut self, node: NodeDir<T>);
    /// Gets a  left node from a node
    fn get_left(&self)-> NodeDir<T>;
    /// Gets a right node from a node
    fn get_right(&self)-> NodeDir<T>;
    /// Checks if there is a right node
    fn is_right(&self) -> bool;
    /// Checks if there is a left node
    fn is_left(&self)->bool;
}
pub struct BST<T>{
    root: NodeDir<T>,
    pub count: u32
}
impl<T> Tree<T> for BST<T>{
    /// Function to set the left-most branch node to another NodeDir
    fn set_left(&mut self, node: NodeDir<T>){
        // c_node is the current node selected on the BST
        let mut c_node = Rc::clone(self.root.as_ref().unwrap());
        Rc::try_unwrap(c_node).ok().unwrap().into_inner().left = node
    }
    /// Function to set the right-most branch node to another NodeDir
    fn set_right(&mut self, node: NodeDir<T>){
        // c_node is the current node selected on the BST
        let mut c_node = Rc::clone(self.root.as_ref().unwrap());
        Rc::try_unwrap(c_node).ok().unwrap().into_inner().right = node
    }

    fn get_left(&self) -> NodeDir<T>{
        let temp = Rc::clone(self.root.as_ref().unwrap());
        Rc::try_unwrap(temp).ok().unwrap().into_inner().left
    }

    fn get_right(&self) -> NodeDir<T>{
        let temp = Rc::clone(self.root.as_ref().unwrap());
        Rc::try_unwrap(temp).ok().unwrap().into_inner().right
    }

    fn is_right(&self) -> bool {
        let c_node = Rc::clone(self.root.as_ref().unwrap());
        Rc::try_unwrap(c_node).ok().unwrap().into_inner().right.is_some()
    }

    fn is_left(&self) -> bool {
        let c_node = Rc::clone(self.root.as_ref().unwrap());
        Rc::try_unwrap(c_node).ok().unwrap().into_inner().left.is_some()
    }
}
impl<T> BST<T>{
    pub fn new()->Self{
        BST {
            root: None,
            count: 0
        }
    }
    pub fn is_empty(&self)-> bool{self.count == 0}
    fn new_node_dir(&self, key:u32, data: T) -> NodeDir<T>{
        Some(Rc::new(RefCell::new(Node::new(key,data))))
    }
    pub fn add(&mut self, key:u32, data: T){
        if self.is_empty(){
            self.root = self.new_node_dir(key, data);
            self.count += 1;
            return;
        }
        let kv_pair = (key, data);
        let mut curr = Rc::clone(self.root.as_ref().unwrap());


    }
    fn _recuradd(&mut self, kv_p:(u32, T), node: Node<T>){
        if kv_p.0 < node.key{
           if let None = node.left{

           }else{

           }
        }else if kv_p.0 > node.key{
            if let None = node.right{

            }else{

            }
        }
    }
}