struct FibonacciState {
    first: u32,
    second: u32,
    even_sum: u32,
}

impl Default for FibonacciState {
    fn default() -> Self {
        Self {
            first: 1,
            second: 2,
            even_sum: 2,
        }
    }
}

impl FibonacciState {
    fn add_next(&mut self) {
        let new = self.first + self.second;
        self.first = self.second;
        self.second = new;
        if new % 2 == 0 {
            self.even_sum += new;
        }
    }
    fn is_limit_reached(&self) -> bool {
        self.second >= 4_000_000
    }
}

pub fn main() {
    let mut fibonacci_state = FibonacciState::default();

    while !fibonacci_state.is_limit_reached() {
        fibonacci_state.add_next();
    }

    println!("Sum: {}", fibonacci_state.even_sum);
}
