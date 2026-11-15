use std::cmp::Ordering::*;
struct MinStack {
    st: Vec<i64>,
    min: i64,
}
impl MinStack {
    pub fn new() -> Self {
        Self {
            st: vec![],
            min: i64::MAX,
        }
    }
    pub fn push(&mut self, val: i32) {
        let val = val as i64;
        if self.st.is_empty() {
            // difference between val and min(st) is 0
            self.st.push(0);
        } else {
            // push the diff between val and old min
            // val is new min iff this is negative, i.e. new min - old min
            self.st.push(val - self.min);
        }
        self.min = self.min.min(val);
    }
    pub fn pop(&mut self) {
        let top = self.st.pop().unwrap();
        if self.st.is_empty() {
            self.min = i64::MAX;
        } else if top < 0 {
            // top is (encoded as) new min - old min, thus we restore old min
            self.min -= top;
        }
    }
    pub fn top(&self) -> i32 {
        let &top = self.st.last().unwrap();
        (if top < 0 {
            // top is min
            self.min
        } else {
            top + self.min
        }) as i32
    }
    pub fn get_min(&self) -> i32 {
        self.min as i32
    }
}
