// iterators1.rs
//
// 迭代器是Rust中处理集合元素的重要工具
// 这个文件展示了迭代器的基本用法和常见错误

fn main() {
    // 创建一个水果数组
    let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];
    
    println!("=== 正确的迭代器用法 ===");
    // 创建一个可变迭代器
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();
    
    // 逐个访问元素
    println!("第一个元素: {:?}", my_iterable_fav_fruits.next());
    println!("第二个元素: {:?}", my_iterable_fav_fruits.next());
    println!("第三个元素: {:?}", my_iterable_fav_fruits.next());
    println!("第四个元素: {:?}", my_iterable_fav_fruits.next());
    println!("第五个元素: {:?}", my_iterable_fav_fruits.next());
    println!("迭代结束: {:?}", my_iterable_fav_fruits.next());
    
    // 重置迭代器
    let mut another_iter = my_fav_fruits.iter();
    
    // 断言测试
    assert_eq!(another_iter.next(), Some(&"banana"));
    assert_eq!(another_iter.next(), Some(&"custard apple"));
    assert_eq!(another_iter.next(), Some(&"avocado"));
    assert_eq!(another_iter.next(), Some(&"peach"));
    assert_eq!(another_iter.next(), Some(&"raspberry"));
    assert_eq!(another_iter.next(), None);
    
    println!("\n=== 迭代器常见错误示例 ===");
    println!("1. 忘记声明mut：如果尝试在非mut迭代器上调用next()方法，会导致编译错误");
    // let non_mut_iter = my_fav_fruits.iter();
    // non_mut_iter.next(); // 编译错误：不能在不可变引用上调用可变方法
    
    println!("2. 迭代器耗尽：一旦迭代器到达末尾，再次调用next()将始终返回None");
    let mut exhausted_iter = my_fav_fruits.iter();
    // 消耗迭代器
    for _ in exhausted_iter.by_ref() {
        // 空操作，只是消耗迭代器
    }
    println!("耗尽后的迭代器: {:?}", exhausted_iter.next());
    
    println!("3. 所有权问题：into_iter()会获取所有权，之后原始集合将不可用");
    let fruits = vec!["apple", "orange"];
    let _fruit_iter = fruits.into_iter(); // 添加下划线前缀表示故意未使用
    // 下面这行如果取消注释会导致编译错误，因为fruits的所有权已经转移给了迭代器
    // println!("{:?}", fruits);
    
    println!("\n成功完成迭代器练习！");
}
