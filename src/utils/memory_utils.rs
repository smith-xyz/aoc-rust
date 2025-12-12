use std::{cmp::Reverse, collections::BinaryHeap};

pub fn create_memory_heap_from_vec<T, R, F>(data: &[T], mut callback: F) -> BinaryHeap<R>
where
    R: Ord,
    F: FnMut(&T, &T, usize, usize) -> R,
{
    let mut heap = BinaryHeap::new();
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let result = callback(&data[i], &data[j], i, j);
            heap.push(result);
        }
    }
    heap
}

pub fn create_memory_min_heap_from_vec<T, R, F>(
    data: &[T],
    mut callback: F,
) -> BinaryHeap<Reverse<R>>
where
    R: Ord,
    F: FnMut(&T, &T, usize, usize) -> R,
{
    let mut heap = BinaryHeap::new();
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let result = callback(&data[i], &data[j], i, j);
            heap.push(Reverse(result));
        }
    }
    heap
}
