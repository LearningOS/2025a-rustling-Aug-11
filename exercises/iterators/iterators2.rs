// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            // 将第一个字符大写，然后添加剩余的字符
            let mut result = String::new();
            result.push(first.to_uppercase().next().unwrap_or(first));
            result.push_str(c.as_str());
            result
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 使用迭代器将capitalize_first应用到每个单词
    words.iter().map(|&word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // 使用迭代器将capitalize_first应用到每个元素，然后连接成一个字符串
    words.iter().map(|&word| capitalize_first(word)).collect()
}

fn main() {
    // 示例用法
    println!("capitalize_first(\"hello\") = {}", capitalize_first("hello"));
    println!("capitalize_first(\"\") = {}", capitalize_first(""));
    
    let words = vec!["hello", "world"];
    println!("capitalize_words_vector({:?}) = {:?}", words, capitalize_words_vector(&words));
    
    let phrase = vec!["hello", " ", "world"];
    println!("capitalize_words_string({:?}) = {:?}", phrase, capitalize_words_string(&phrase));
    
    println!("成功完成迭代器练习！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
