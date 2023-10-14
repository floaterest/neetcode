use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    fn iut(root: &Option<Rc<RefCell<TreeNode>>>) -> HashSet<i32> {
        root.iter()
            .flat_map(|r| {
                let r = r.borrow();
                Self::iut(&r.left)
                    .into_iter()
                    .chain(Self::iut(&r.right).into_iter())
                    .chain(std::iter::once(r.val))
            })
            .collect()
    }
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::iut(&root).len() == 1
    }
}

