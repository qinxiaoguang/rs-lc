use super::*;
use std::rc::Rc;
use std::{cell::RefCell, collections::VecDeque};

struct CBTInserter {
    queue: VecDeque<Rc<RefCell<TreeNode>>>,
    root: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut tmp_queue = VecDeque::new();
        let mut queue = VecDeque::new();
        tmp_queue.push_back(root.clone().unwrap());

        // bfs遍利节点，并把节点中所有数据插入到队列中
        while !tmp_queue.is_empty() {
            let pop = tmp_queue.pop_front().unwrap();
            if (*pop).borrow().left == None || (*pop).borrow().right == None {
                queue.push_back(pop.clone());
            }
            if (*pop).borrow().left != None {
                tmp_queue.push_back((*pop).borrow().left.clone().unwrap());
            }
            if (*pop).borrow().right != None {
                tmp_queue.push_back((*pop).borrow().right.clone().unwrap());
            }
        }

        Self {
            queue: queue,
            root: root,
        }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let rc = Rc::new(RefCell::new(TreeNode {
            val: v,
            left: None,
            right: None,
        }));

        self.queue.push_back(rc.clone());
        if let Some(front) = self.queue.front() {
            let res = (*front).borrow().val;
            if (*front).borrow().left == None {
                (*front).borrow_mut().left = Some(rc.clone());
                return res;
            }
            if (*front).borrow().right == None {
                (*front).borrow_mut().right = Some(rc.clone());
                self.queue.pop_front();
                return res;
            }
        }
        unreachable!();
        return 0;
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_043() {}
}
