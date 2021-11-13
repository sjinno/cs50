use labs::sort;

fn main() {
    // let mut arr = vec![
    //     5.2, 7.2, 1.2, 3.2, 2.2, 54.2, 211.2, 4.2, 6.2, 7.2, 0.2, 12.2, -76.2, 2.2,
    // ];
    // sort::bubble_sort(&mut arr);
    // eprintln!("{:?}", arr);

    // let mut arr = vec![5, 4, 3, 2, 1];
    // sort::selection_sort(&mut arr);
    // eprintln!("{:?}", arr);

    let mut arr = vec![7, 5, 1, 6, 2];
    sort::merge_sort(&mut arr);
    eprintln!("{:?}", arr);
}
