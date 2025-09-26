// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    // 使用泛型创建字符串向量作为购物列表
    let mut shopping_list: Vec<&str> = vec!["milk"];
    
    // 添加更多商品，展示mutability
    shopping_list.push("eggs");
    shopping_list.push("bread");
    
    // 使用购物列表，避免未使用变量警告
    println!("Shopping List:");
    for (index, item) in shopping_list.iter().enumerate() {
        println!("{}. {}", index + 1, item);
    }
    
    println!("Total items: {}", shopping_list.len());
}
