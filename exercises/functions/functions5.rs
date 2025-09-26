// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// 定义一个自定义错误类型
#[derive(Debug)]
enum SquareError {
    Overflow(i32),        // 整数溢出
    NegativeNumber(i32),  // 负数输入（某些应用场景可能不允许）
}

// 实现Display trait来提供错误描述
impl std::fmt::Display for SquareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SquareError::Overflow(num) => write!(
                f, "Integer overflow occurred when squaring {}. Result exceeds i32 range.", num
            ),
            SquareError::NegativeNumber(num) => write!(
                f, "Cannot square negative number: {}. This operation requires non-negative input.", num
            ),
        }
    }
}

// 实现Error trait
impl std::error::Error for SquareError {}

fn main() {
    // 测试正常情况
    match safe_square(3) {
        Ok(result) => println!("The square of 3 is {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 测试错误情况
    println!("\nTesting error cases:");
    
    // 测试负数输入
    match safe_square(-5) {
        Ok(result) => println!("The square of -5 is {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 测试可能的溢出情况（使用接近i32最大值的数字）
    let large_num = 46340; // 这个数的平方接近i32的最大值(2^31-1)
    match safe_square(large_num) {
        Ok(result) => println!("The square of {} is {}", large_num, result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 测试肯定会溢出的情况
    match safe_square(46341) {
        Ok(result) => println!("The square of 46341 is {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// 添加带有错误处理的安全版本函数
fn safe_square(num: i32) -> Result<i32, Box<dyn std::error::Error>> {
    // 检查是否为负数（根据业务需求，这里假设不允许负数）
    if num < 0 {
        return Err(Box::new(SquareError::NegativeNumber(num)));
    }
    
    // 检查是否会导致溢出
    // 使用i32的最大值的平方根作为阈值
    // i32::MAX is 2147483647, its square root is approximately 46340.95
    if num > 46340 {
        return Err(Box::new(SquareError::Overflow(num)));
    }
    
    // 如果安全，则计算平方
    Ok(num * num)
}
