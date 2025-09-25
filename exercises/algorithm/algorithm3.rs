/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T: Ord>(array: &mut [T]){
	// 如果数组长度小于等于1，则已经排序好
	if array.len() <= 1 {
		return;
	}
	
	// 调用快速排序辅助函数
	quick_sort(array, 0, (array.len() - 1) as isize);
}

fn quick_sort<T: Ord>(array: &mut [T], low: isize, high: isize) {
	if low < high {
		// 获取分区点
		let pivot_index = partition(array, low, high);
		// 递归排序分区点左边的元素
		quick_sort(array, low, pivot_index - 1);
		// 递归排序分区点右边的元素
		quick_sort(array, pivot_index + 1, high);
	}
}

fn partition<T: Ord>(array: &mut [T], low: isize, high: isize) -> isize {
	// 选择最右边的元素作为基准
	let pivot_index = high as usize;
	let mut i = low - 1;
	
	for j in low..high {
		// 如果当前元素小于或等于基准元素
		if array[j as usize] <= array[pivot_index] {
			i += 1;
			// 交换元素
			array.swap(i as usize, j as usize);
		}
	}
	
	// 将基准元素放到正确的位置
	array.swap((i + 1) as usize, pivot_index);
	
	// 返回基准元素的索引
	i + 1
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}