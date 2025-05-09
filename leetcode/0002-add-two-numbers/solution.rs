type L = Option<Box<ListNode>>;
impl Solution {
    pub fn divmod(a: i32, b: i32) -> (i32, i32) {
        (a / b, a % b)
    }
    pub fn f(l1: L, l2: L, n: i32) -> L {
        let a = l1.iter().map(|v| v.val).next().unwrap_or(0);
        let b = l2.iter().map(|v| v.val).next().unwrap_or(0);
        let val = a + b + n;
        if val == 0 && l1.is_none() && l2.is_none() {
            return None;
        }
        let (n, val) = Self::divmod(a + b + n, 10);
        Some(Box::new(ListNode {
            val,
            next: Self::f(l1.and_then(|v| v.next), l2.and_then(|v| v.next), n),
        }))
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match Self::f(l1, l2, 0) {
            Some(a) => Some(a),
            None => Some(Box::new(ListNode { val: 0, next: None })),
        }
    }
}


