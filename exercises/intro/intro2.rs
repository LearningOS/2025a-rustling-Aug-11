// intro2.rs
//
// 这个文件包含一些Rust常见错误示例
// 取消注释下面的代码来查看不同类型的错误
//

fn main() {
    // 1. 编译错误示例：未声明的变量
    // println!("{} is not declared", undeclared_variable);
    
    // 2. 编译错误示例：类型不匹配
    // let x: i32 = "This is a string";
    
    // 3. 编译错误示例：缺少分号
    // let y = 5
    
    // 4. 编译错误示例：括号不匹配
    // if true {
    //     println!("Missing closing bracket");
    
    // 5. 运行时错误示例：除零错误
    // let result = 10 / 0;
    
    // 6. 运行时错误示例：数组越界
    // let arr = [1, 2, 3];
    // println!("{}", arr[10]);
    
    // 当前正确的代码
    println!("Hello world!");
    println!("要查看错误示例，请取消上面相应行的注释");
    println!("常见错误类型：");
    println!("- 编译错误：语法错误、类型不匹配、未声明的变量");
    println!("- 运行时错误：除零错误、数组越界、空指针解引用");
}
