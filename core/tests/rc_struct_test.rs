#[cfg(test)]
mod rc_bst_test {
    use textexcelport_core::structures::rcBST::{BST, Tree, Node};

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
        assert_eq!(bt_tree.count, 3);
        assert_eq!(bt_tree.is_right(), true);
        assert_eq!(bt_tree.is_left(), true);
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
        let a = bt_tree.get_right();
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
}