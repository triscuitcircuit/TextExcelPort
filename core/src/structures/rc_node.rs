use std::rc::Rc;
use std::cell::RefCell;

///Optional reference to another Node of generic T type
pub type NodeDir<T> = Option<Rc<RefCell<Node<T>>>>;


pub trait Tree<T: Clone>  {
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


///Node with generic T type.
///Key is separated from data in case generic doesnt have std::cmp
///
/// right and left nodes connecting to it are optional references
#[derive(Debug)]
pub struct Node<T: Clone>  {
    pub key: u32,
    data: T,
    pub left: NodeDir<T>,
    pub right: NodeDir<T>
}
impl<T: Clone> Node<T>  {
    pub fn new(key: u32, data: T)-> Self{
        Node{
            key,
            data,
            left: None,
            right: None
        }
    }
    pub fn get_key(&self) ->u32{
        self.key
    }
    pub fn get_data(&self)->T{
        self.data.clone()
    }
}
impl<T: Clone> Tree<T> for Node<T>  {
    /// Function to set the left-most branch node to another NodeDir
    fn set_left(&mut self, node: NodeDir<T>){
        // c_node is the current node selected on the Node
        if let None = self.left{
            self.left = node
        }
    }
    /// Function to set the right-most branch node to another NodeDir
    fn set_right(&mut self, node: NodeDir<T>){
        if let None = self.right{
            self.right = node
        }
    }
    fn get_left(&self) -> NodeDir<T>{
        self.left.clone()
    }
    fn get_right(&self) -> NodeDir<T>{
        self.right.clone()
    }
    fn is_right(&self) -> bool {
        self.right.is_some()
    }
    fn is_left(&self) -> bool {
        self.left.is_some()
    }
}