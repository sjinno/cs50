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
        for i in 1..arr.len() {
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

//  0  1  2  3  4
// [7, 5, 1, 6, 2]
pub fn merge_sort<T: Clone + PartialOrd + ToOwned<Owned = T> + Debug>(arr: &[T]) -> Vec<T> {
    eprintln!("{:?}", arr);

    if arr.len() == 1 {
        return arr.to_vec();
    }

    let split_point = arr.len() / 2; // 5 / 2 = 2
    let chunk1 = merge_sort(&arr[0..split_point]); // L[7, 5] -> L[7], R[5]
    let chunk2 = merge_sort(&arr[split_point..]); // R[1, 6, 2] -> L[1], R[6, 2] -> L[6], R[2]

    eprintln!("chunk1: {:?}", chunk1);
    eprintln!("chunk2: {:?}", chunk2);

    let mut l = 0;
    let mut r = 0;
    let mut v = Vec::new();

    while l < chunk1.len() && r < chunk2.len() {
        eprintln!("left: {}", l);
        eprintln!("right: {}", r);

        eprintln!("{:?}", chunk1[l]);
        eprintln!("{:?}", chunk2[r]);

        if chunk1[l] < chunk2[r] {
            v.push(chunk1[l].to_owned());
            l += 1;
        } else {
            v.push(chunk2[r].to_owned());
            r += 1;
        }
    }

    if l != chunk1.len() {
        v.extend_from_slice(&chunk1[l..]);
    } else {
        v.extend_from_slice(&chunk2[r..]);
    }

    eprintln!("VVVVVVVVVV {:?}", v);
    v
}
