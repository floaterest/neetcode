impl Solution {
    pub fn length_of_lis(ns: Vec<i32>) -> i32 {
        let mut arr = Vec::with_capacity(ns.len());
        for n in &ns {
            match arr.binary_search(n) {
                Ok(_) => {}
                Err(i) => {
                    if i == arr.len() {
                        arr.push(*n);
                    } else {
                        arr[i] = *n;
                    }
                }
            }
        }
        arr.len() as i32
    }
}

