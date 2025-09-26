// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// 使用泛型参数T来支持任何类型
struct Wrapper<T> {
    value: T,
}

// 为泛型结构体实现方法
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// 添加main函数作为程序入口点
fn main() {
    // 简单的main函数，可以留空或添加一些示例代码
    let wrapper = Wrapper::new(42);
    println!("Wrapper contains: {}", wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
