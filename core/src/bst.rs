use std::{
    cell::RefCell,cmp::Ordering, collections::VecDeque,
    env,io::{self,Write},path::Path,rc::Rc
};
pub struct Node<T>{
    pub key: u32,
    data: T
}
pub struct Tree<T>{
    root: Option<Rc<RefCell<Node<T>>>>
}