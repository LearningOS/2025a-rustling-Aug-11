// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// 定义一个自定义错误类型
#[derive(Debug)]
enum CallError {
    TooManyCalls(u32), // 调用次数过多
    InvalidCall,
}

// 实现Display trait来提供错误描述
impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallError::TooManyCalls(n) => write!(f, "Too many calls requested: {}. Maximum is 10.", n),
            CallError::InvalidCall => write!(f, "Invalid call request"),
        }
    }
}

// 实现Error trait
impl std::error::Error for CallError {}

fn main() {
    // 测试正常情况
    match call_me_safe(3) {
        Ok(_) => println!("Calls completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 测试错误情况 - 调用次数过多
    match call_me_safe(15) {
        Ok(_) => println!("Calls completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 测试错误情况 - 特殊的"禁止"数字
    match call_me_safe(7) {
        Ok(_) => println!("Calls completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 演示panic!的使用
    println!("\nDemonstrating panic! for invalid input:");
    call_me_panic(0); // 这会触发panic
}

// 安全版本:使用Result返回错误
fn call_me_safe(num: u32) -> Result<(), Box<dyn std::error::Error>> {
    // 检查调用次数是否过多
    if num > 10 {
        return Err(Box::new(CallError::TooManyCalls(num)));
    }
    
    // 添加对InvalidCall变体的使用场景
    // 例如，特殊的"禁止"数字
    if num == 7 { // 假设7是一个特殊的"禁止"数字
        return Err(Box::new(CallError::InvalidCall));
    }
    
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
    
    Ok(())
}

// 演示panic!的使用
fn call_me_panic(num: u32) {
    // 对于特殊情况使用panic!
    if num == 0 {
        panic!("Cannot make zero calls - this is a critical error!");
    }
    
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
