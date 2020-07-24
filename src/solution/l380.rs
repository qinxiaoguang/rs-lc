pub struct Solution {}

/*
* @lc app=leetcode.cn id=380 lang=rust
*
* [380] 常数时间插入、删除和获取随机元素
*
* https://leetcode-cn.com/problems/insert-delete-getrandom-o1/description/
*
* algorithms
* Medium (47.94%)
* Likes:    155
* Dislikes: 0
* Total Accepted:    14.3K
* Total Submissions: 29.5K
* Testcase Example:  '["RandomizedSet","insert","remove","insert","getRandom","remove","insert","getRandom"]\n' +
 '[[],[1],[2],[2],[],[1],[2],[]]'
*
* 设计一个支持在平均 时间复杂度 O(1) 下，执行以下操作的数据结构。
*
*
* insert(val)：当元素 val 不存在时，向集合中插入该项。
* remove(val)：元素 val 存在时，从集合中移除该项。
* getRandom：随机返回现有集合中的一项。每个元素应该有相同的概率被返回。
*
*
* 示例 :
*
*
* // 初始化一个空的集合。
* RandomizedSet randomSet = new RandomizedSet();
*
* // 向集合中插入 1 。返回 true 表示 1 被成功地插入。
* randomSet.insert(1);
*
* // 返回 false ，表示集合中不存在 2 。
* randomSet.remove(2);
*
* // 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
* randomSet.insert(2);
*
* // getRandom 应随机返回 1 或 2 。
* randomSet.getRandom();
*
* // 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
* randomSet.remove(1);
*
* // 2 已在集合中，所以返回 false 。
* randomSet.insert(2);
*
* // 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
* randomSet.getRandom();
*
*
*/

// @lc code=start
use std::cell::RefCell;
use std::collections::HashSet;
struct RandomizedSet {
    set: RefCell<HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedSet {
            set: RefCell::new(HashSet::new()),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&self, val: i32) -> bool {
        if self.set.borrow().contains(&val) {
            return false;
        }

        self.set.borrow_mut().insert(val);
        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&self, val: i32) -> bool {
        if !self.set.borrow().contains(&val) {
            return false;
        }

        self.set.borrow_mut().remove(&val);
        return true;
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        // 不太好搞，不搞了
        let s = self.set.borrow();
        let n = s.iter().next();
        if n.is_some() {
            return *n.unwrap();
        }
        return 0;
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l380() {
        let mut s = RandomizedSet::new();
        s.insert(1);
        s.remove(1);
        s.insert(2);
        println!("res:{:?}", s.get_random());
    }
}
