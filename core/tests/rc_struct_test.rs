#[cfg(test)]
mod rc_bst_test {
    use textexcelport_core::structures::rcBST::{BST};

    #[test]
    fn add_one_count() {
        let a = String::from("String");
        let mut bt_tree: BST<String> = BST::new();
        bt_tree.add(0, a);
        assert_eq!(bt_tree.count, 1);

    }

    #[test]
    fn add_multiple_count() {
        let mut bt_tree: BST<String> = BST::new();
        bt_tree.add(25, String::from("test_node1"));
        bt_tree.add(1, String::from("test_node2"));
        bt_tree.add(26, String::from("test_node3"));
        let mut b: Vec<String> = Vec::new();
        bt_tree.inorder_sort(&mut b);
        assert_eq!(bt_tree.count, 3);
        assert_eq!(bt_tree.is_right(), true);
        assert_eq!(b,["test_node2","test_node1","test_node3"]);
        assert_eq!(bt_tree.is_left(), true);
    }
    #[test]
    fn find_node_test(){
        let mut bt_tree: BST<String> = BST::new();
        bt_tree.add(25, String::from("test_node1"));
        bt_tree.add(1, String::from("test_node2"));
        bt_tree.add(26, String::from("test_node3"));
        let temp:Option<u32> = match bt_tree.find(26){
            None => None,
            Some(e) => {
                Some(e.borrow().key.clone())
            }
        };
        assert_eq!(temp.unwrap(),26);

    }

    #[test]
    fn add_one_node() {
        let mut bt_tree: BST<String> = BST::new();
        bt_tree.add(0, String::from("test1-root"));
        bt_tree.add(1, String::from("test2-node"));
        assert_eq!(bt_tree.is_right(), true);
        assert_eq!(bt_tree.count, 2);
    }

    #[test]
    fn add_multiple_node() {
        let mut bt_tree: BST<String> = BST::new();
        bt_tree.add(25, String::from("test_node1"));
        bt_tree.add(1, String::from("test_node2"));
        assert_eq!(bt_tree.is_left(), true);
    }
    #[test]
    fn add_25_nodes(){
        let mut bst_tree: BST<String> = BST::new();
        for i in 0..25{
            bst_tree.add(i,String::from(format!("test_node{}",i)))
        }
        assert_eq!(bst_tree.count, 25);
        assert_eq!(bst_tree.is_right(), true);
        let temp:Option<u32> = match bst_tree.find(22){
            None => None,
            Some(e) => {
                Some(e.borrow().key.clone())
            }
        };
        assert_eq!(temp.unwrap(),22);
    }
}