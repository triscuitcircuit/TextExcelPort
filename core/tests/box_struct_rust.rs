#[cfg(test)]
mod box_bst_test {
    use textexcelport_core::structures::boxBST::{BST};

    #[test]
    fn add_one_count() {
        let a = String::from("String");
        let mut bt_tree: BST<String> = BST::new();
        bt_tree.add(0, a);
        assert_eq!(bt_tree.count, 1);
    }
}