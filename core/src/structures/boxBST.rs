///Node with generic T type.
///Key is separated from data in case generic doesnt have std::cmp
///
/// right and left nodes connecting to it are optional references
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T>  {
    pub key: u32,
    data: T,
    pub left: Link<T>,
    pub right: Link<T>,
    pub parent: Link<T>
}
impl<T> Node<T>{}

impl<T>Node<T>{
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
pub struct BST<T>  {
    root: Link<T>,
    pub count: u32
}
impl<T> BST<T>{
    pub fn new() -> Self{
        BST{
            root: None,
            count: 0
        }
    }
    pub fn is_empty(&self)->bool{self.count==0}
    pub fn add(&mut self, key: u32, data: T){
        if self.is_empty(){
            self.root = Some(Box::new(Node{
                key,
                data,
                left: None,
                right: None,
                parent: None
            }));
            self.count = 1;
            return;
        }
        let kv_pair = (key,data);
        self.count +=1 ;

    }
}

