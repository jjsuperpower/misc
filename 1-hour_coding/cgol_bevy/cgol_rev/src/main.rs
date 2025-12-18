/// Reverse Conway's Game of Life
///
/// This library provides algorithms to compute possible previous states
/// that could have led to a given Conway's Game of Life pattern.
///
/// Conway's Game of Life is deterministic going forward, but when going
/// backward, there can be multiple valid previous states that could have
/// produced the current pattern.

pub fn main() {
    println!("Conway's Game of Life Reverse Solver");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
