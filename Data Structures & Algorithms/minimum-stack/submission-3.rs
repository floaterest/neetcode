struct MinStack {
    st: Vec<(i32, i32)>,
}
impl MinStack {
    pub fn new() -> Self {
        Self { st: vec![] }
    }
    pub fn push(&mut self, val: i32) {
        self.st.push(if let Some(&(_, min)) = self.st.last() {
            (val, min.min(val))
        } else {
            // empty stack
            (val, val)
        })
    }
    pub fn pop(&mut self) {
        self.st.pop();
    }
    pub fn top(&self) -> i32 {
        self.st.last().unwrap().0
    }
    pub fn get_min(&self) -> i32 {
        self.st.last().unwrap().1
    }
}
