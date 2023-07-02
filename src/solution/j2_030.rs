use super::Solution;
use super::*;
use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    m: HashMap<i32, usize>,
    arr: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            m: HashMap::new(),
            arr: Vec::new(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.m.contains_key(&val) {
            return false;
        }
        self.arr.push(val);
        self.m.insert(val, self.arr.len() - 1);
        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.arr.len() == 0 {
            return false;
        }
        if !self.m.contains_key(&val) {
            return false;
        }
        // 存在
        let idx = self.m.get(&val).unwrap();
        self.arr[*idx] = self.arr[self.arr.len() - 1];
        self.m.insert(self.arr[*idx], *idx);
        self.arr.pop();
        self.m.remove(&val);
        return true;
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut i: usize = rng.gen();
        i = i % self.m.len();
        return self.arr[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_030() {}
}
