// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.

// 定义一个自定义错误类型
#[derive(Debug)]
enum PriceError {
    NegativePrice(i32),       // 价格为负数
    NegativeSalePrice(i32, i32), // 折扣后价格为负数(原价, 折扣后价格)
}

// 实现Display trait来提供错误描述
impl std::fmt::Display for PriceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PriceError::NegativePrice(price) => write!(f, "Price cannot be negative: {}", price),
            PriceError::NegativeSalePrice(original, sale) => write!(
                f, "Sale price cannot be negative. Original price: {}, Discount would result in: {}", 
                original, sale
            ),
        }
    }
}

// 实现Error trait
impl std::error::Error for PriceError {}

fn main() {
    let original_price = 51;
    
    match calculate_sale_price(original_price) {
        Ok(price) => println!("Your sale price is {}", price),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 测试错误情况
    println!("\nTesting error cases:");
    
    // 测试负数价格
    match calculate_sale_price(-10) {
        Ok(price) => println!("Your sale price is {}", price),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // 测试折扣后价格为负数的情况
    match calculate_sale_price(8) {
        Ok(price) => println!("Your sale price is {}", price),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// 原始的sale_price函数保持不变
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

// 添加带有错误处理的新版本函数
fn calculate_sale_price(price: i32) -> Result<i32, Box<dyn std::error::Error>> {
    // 检查价格是否为负数
    if price < 0 {
        return Err(Box::new(PriceError::NegativePrice(price)));
    }
    
    let discounted_price = sale_price(price);
    
    // 检查折扣后价格是否为负数
    if discounted_price < 0 {
        return Err(Box::new(PriceError::NegativeSalePrice(price, discounted_price)));
    }
    
    Ok(discounted_price)
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
