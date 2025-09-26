// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

pub fn factorial(num: u64) -> u64 {
    // 使用迭代器计算阶乘
    // 1. 如果num为0或1，直接返回1
    // 2. 否则使用迭代器从1到num生成序列并计算乘积
    (1..=num).product()
}

fn main() {
    // 示例用法
    println!("factorial(0) = {}", factorial(0));
    println!("factorial(1) = {}", factorial(1));
    println!("factorial(2) = {}", factorial(2));
    println!("factorial(3) = {}", factorial(3));
    println!("factorial(4) = {}", factorial(4));
    println!("factorial(5) = {}", factorial(5));
    println!("成功完成阶乘练习！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
