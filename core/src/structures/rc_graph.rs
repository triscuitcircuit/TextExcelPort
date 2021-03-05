use std::rc::Rc;

#[derive(Debug)]
pub struct Graph<T: Clone + ::std::fmt::Debug>{
    elem: T,
    origin: Option<Rc<Graph<T>>>,
    dest: Option<Rc<Graph<T>>>
}

impl<T: Clone + ::std::fmt::Debug> Graph<T> {
    fn new(data: T) -> Self{
        Self{
            elem: data,
            origin:None,
            dest: None
        }
    }
    fn set_dest(&mut self, dest: Graph<T>){

    }
    fn set_origin(&mut self, origin: Graph<T>){

    }
}