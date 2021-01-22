#![feature(test)]
extern crate test;
use textexcelport_core::structures::rc_bst::{BST};
use test::Bencher;

#[bench]
fn add_256_test(b: &mut Bencher) {
    let a = 2;
    let mut bt_tree: BST<i32> = BST::new();
    b.iter(||{
        for i in 0..256{
            bt_tree.add(i, a);
        }
    });
}

#[bench]
fn add_one_test(b: &mut Bencher) {
    let a = 2;
    let mut bt_tree: BST<i32> = BST::new();
    b.iter(||{
            bt_tree.add(0, a);
    });
}
#[bench]
fn sort_256_test(b: &mut Bencher) {
    let a = 2;
    let mut bt_tree: BST<i32> = BST::new();
    for i in 0..256{
        bt_tree.add(i, a);
    }
    let mut arr:Vec<i32> = Vec::new();
    b.iter(||{
        bt_tree.inorder_sort(& mut arr);
    });
}