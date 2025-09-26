// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // 声明一个HashMap，键为String类型，值为u32类型
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)    
    basket.insert(String::from("banana"), 2);

    // 添加更多水果，确保至少3种不同水果，总数至少5个
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);

    basket
}

// 添加main函数作为程序入口点
fn main() {
    let basket = fruit_basket();
    println!("Fruit basket contents:");
    for (fruit, count) in &basket {
        println!("- {}: {}", fruit, count);
    }
    println!("Total fruits: {}", basket.values().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
