// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.


pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional code

    if a > b {
        a
    } else {
        b
    }
}

// Don't mind this for now :)

fn main() {
    let a = 10;
    let b = 20;
    let result = bigger(a, b);
    println!("较大的数是: {}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
