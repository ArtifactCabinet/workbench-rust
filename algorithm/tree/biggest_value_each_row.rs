use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

// TODO: fix this by convert [1, 3,2, 5,3,9,none] to tree
// TODO: lear Box<> type to replace Rc<RefCell>

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}


fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut q = vec![root];
    let mut q2 = vec![];
    let mut n = q.len();
    
    while n > 0 {
        let mut hi = std::i32::MIN;
        let mut has_node = false;
        for root in &q {
            if let Some(node) = root {
                // node `&std::rc::Rc<std::cell::RefCell<tree_node::TreeNode>>`
                // v TreeNode, or `std::cell::Ref<'_, tree_node::TreeNode>`.
                has_node = true;
                let v = node.borrow_mut();
                hi = max(hi, v.val);
                q2.push(v.left.clone());
                q2.push(v.right.clone());
            }
        }
        if has_node {
            res.push(hi);
        }
        q.clear();
        q.append(&mut q2);
        n = q.len();
    }
    res
}


fn main(){
    // find biggest value each row
    //  1
    //  |\
    //  3 2
    //  |\  \
    //  5 3  9
    //  => [1,2,9]
    
    let tree=
      Some(Rc::new(RefCell::new(TreeNode{
          val: 1,
          left:Some(Rc::new(RefCell::new(
            TreeNode{
              val:3,
              left:Some(Rc::new(RefCell::new(
                TreeNode{
                  val:5,
                  left:None,
                  right:None
                }))),
              right:Some(Rc::new(RefCell::new(
                TreeNode{
                  val:3,
                  left:None,
                  right:None
                })))}))),
          right: Some(Rc::new(RefCell::new(
            TreeNode{
              val:2,
              left:Some(Rc::new(RefCell::new(
                TreeNode{
                  val:9,
                  left:None,
                  right:None
                }))),
              right:None
            }))),
      })));

    let res_vec=largest_values(tree);
    for x in res_vec{
      println!("{}",x);
    }

}