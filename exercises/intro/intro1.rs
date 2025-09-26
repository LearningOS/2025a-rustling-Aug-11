// intro1.rs
//
// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise, even
// after you already figured it out. If you got everything working and feel
// ready for the next exercise, remove the `I AM NOT DONE` comment below.
//
// If you're running this using `rustlings watch`: The exercise file will be
// reloaded when you change one of the lines below! Try adding a `println!`
// line, or try changing what it outputs in your terminal. Try removing a
// semicolon and see what happens!
//
// Execute `rustlings hint intro1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    println!("Hello and");
    println!(r#"       welcome to...                      "#);
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!();
    println!("错误信息示例:");
    println!("- 编译错误: 语法错误、类型不匹配、未使用的变量");
    println!("- 运行时错误: 空指针解引用、数组越界、除零错误");
    println!("- 逻辑错误: 算法错误、条件判断错误");
    println!();
    println!("解决方法:");
    println!("1. 仔细阅读错误信息，定位问题所在");
    println!("2. 检查代码语法和逻辑");
    println!("3. 使用 cargo check 快速检查编译错误");
    println!("4. 使用 cargo clippy 检查代码质量");
    println!("5. 添加适当的测试用例验证修复效果");
    println!();
    println!("继续加油，祝你学习Rust愉快！")
}
