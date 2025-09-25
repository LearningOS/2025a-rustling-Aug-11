/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        // 确保items向量有足够的容量
        if self.count >= self.items.len() {
            self.items.resize(self.count + 1, T::default());
        }
        self.items[self.count] = value;
        
        // 向上调整堆（上浮）
        self.sift_up(self.count);
    }
    
    // 向上调整堆（上浮）
    fn sift_up(&mut self, mut idx: usize) {
        let temp = self.items[idx].clone();
        let comparator = self.comparator;
        
        while idx > 1 && comparator(&temp, &self.items[self.parent_idx(idx)]) {
            self.items[idx] = self.items[self.parent_idx(idx)].clone();
            idx = self.parent_idx(idx);
        }
        
        self.items[idx] = temp;
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            left
        }
    }
    
    // 向下调整堆（下沉）
    fn sift_down(&mut self, mut idx: usize) {
        let comparator = self.comparator;
        
        while self.children_present(idx) {
            let smallest = self.smallest_child_idx(idx);
            
            if !comparator(&self.items[smallest], &self.items[idx]) {
                break;
            }
            
            self.items.swap(idx, smallest);
            idx = smallest;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        // 保存堆顶元素
        let result = self.items[1].clone();
        
        // 将最后一个元素移到堆顶
        self.items[1] = self.items[self.count].clone();
        self.count -= 1;
        
        // 如果堆不为空，向下调整堆
        if !self.is_empty() {
            self.sift_down(1);
        }
        
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}