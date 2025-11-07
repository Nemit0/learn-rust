// Reference solution (not compiled by default).

#[derive(Debug, Clone)]
pub struct Fibonacci {
    pub curr: u64,
    pub next: u64,
    pub remaining: usize,
}

impl Fibonacci {
    pub fn new(n: usize) -> Self {
        Fibonacci { curr: 0, next: 1, remaining: n }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 { return None; }
        let out = self.curr;
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        self.remaining -= 1;
        Some(out)
    }
}

pub fn fib_take(n: usize) -> Vec<u64> {
    Fibonacci::new(n).collect()
}

