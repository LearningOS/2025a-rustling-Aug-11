// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// 定义一个自定义错误类型
#[derive(Debug)]
enum CallError {
    NegativeNumber,
    ZeroCalls,
}

// 实现Display trait来提供错误描述
impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallError::NegativeNumber => write!(f, "Cannot make negative calls"),
            CallError::ZeroCalls => write!(f, "Number of calls must be at least 1"),
        }
    }
}

// 实现Error trait
impl std::error::Error for CallError {}

fn main() {
    // 使用match处理可能的错误
    match call_me(3) {
        Ok(_) => println!("Calls completed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// 修改call_me函数返回Result类型，添加错误处理
fn call_me(num: i32) -> Result<(), Box<dyn std::error::Error>> {
    // 检查参数合法性
    if num < 0 {
        return Err(Box::new(CallError::NegativeNumber));
    }
    if num == 0 {
        return Err(Box::new(CallError::ZeroCalls));
    }
    
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
    
    Ok(())
}
