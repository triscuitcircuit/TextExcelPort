///Node with generic T type.
///Key is separated from data in case generic doesnt have std::cmp
///
/// right and left nodes connecting to it are optional references
#[derive(Debug)]
pub struct Node<T: Copy>  {
    pub key: u32,
    data: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub parent: Option<Box<Node<T>>>
}
impl<T: Copy + Eq + Ord + Default>Node<T>{
    pub fn new(key: u32, data: T) -> Self{
        Node{
            key,
            data,
            left: None,
            right: None,
            parent: None
        }
    }
}
/// Public trait that houses methods that are used with Tree classes
#[derive(Debug)]
pub struct BST<T: Copy>  {
    root: Node<T>,
    pub count: u32
}

