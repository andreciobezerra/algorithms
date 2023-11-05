fn insertion_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn insertion_sort_descending<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut j = i;

        while j > 0 && arr[j] > arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut a = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];

    println!("Before sorting: {:?}", a);
    insertion_sort::<i32>(&mut a);
    println!("After sorting: {:?}", a);
}
