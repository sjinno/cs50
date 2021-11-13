use std::{cmp::PartialOrd, fmt::Debug};

fn swap<T: ToOwned<Owned = T>>(arr: &mut Vec<T>, i: usize, j: usize) {
    let tmp = arr[i].to_owned();
    arr[i] = arr[j].to_owned();
    arr[j] = tmp;
}

pub fn bubble_sort<T: PartialOrd + Clone + Debug>(arr: &mut Vec<T>) {
    let mut is_sorted = false;

    while !is_sorted {
        is_sorted = true;

        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                swap(arr, i, i + 1);
                is_sorted = false;
            }
        }
    }
}

// Helper for selection_sort
fn find_index_of_min<T: PartialOrd + ToOwned<Owned = T>>(arr: &[T]) -> usize {
    if arr.len() == 1 {
        0
    } else {
        let mut min = arr[0].to_owned();
        let mut index = 0;
        for (i, _elt) in arr.iter().enumerate().skip(1) {
            if min > arr[i] {
                min = arr[i].to_owned();
                index = i;
            }
        }
        index
    }
}

pub fn selection_sort<T: PartialOrd + Clone + Debug>(arr: &mut Vec<T>) {
    for i in 0..arr.len() - 1 {
        let min_idx = find_index_of_min(&arr[i + 1..]);
        if arr[i] > arr[min_idx + i + 1] {
            swap(arr, i, min_idx + i + 1);
        }
    }
}

pub fn merge_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    if arr.len() == 1 {
        return;
    }

    let split_point = arr.len() / 2;
    merge_sort(&mut arr[..split_point]);
    merge_sort(&mut arr[split_point..]);

    let mut res = Vec::with_capacity(arr.len());

    merge(&arr[..split_point], &arr[split_point..], &mut res);

    arr.copy_from_slice(&res);
}

fn merge<T: Copy + PartialOrd>(chunk1: &[T], chunk2: &[T], res: &mut Vec<T>) {
    let mut l = 0;
    let mut r = 0;

    while l < chunk1.len() && r < chunk2.len() {
        if chunk1[l] < chunk2[r] {
            res.push(chunk1[l]);
            l += 1;
        } else {
            res.push(chunk2[r]);
            r += 1;
        }
    }

    if l < chunk1.len() {
        res.extend_from_slice(&chunk1[l..]);
    }
    if r < chunk2.len() {
        res.extend_from_slice(&chunk2[r..]);
    }
}
