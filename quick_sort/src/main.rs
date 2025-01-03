fn quick_sort<T>(arr: &mut Vec<T>, low: usize, high: usize)
where
    T: Eq + PartialEq + Clone + PartialOrd
{
    if low < high {
        let p = partition(arr, low, high, &|a, b| {a <= b});
        quick_sort(arr, low, p - 1);
        quick_sort(arr, p + 1, high);
    }
}

fn partition<T, F>(arr: &mut Vec<T>, low: usize, high: usize, f: &F) -> usize
where
    T: Eq + PartialOrd + Clone + PartialOrd,
    F: Fn(&T, &T) -> bool
{
    let pivot = match arr.get(high) {
        Some(v) => v.clone(),
        _ => panic!("Array index out of bounds"),
    };

    let mut i = low;

    for j in low..high - 1 {
        match arr.to_vec().get(j) {
            Some(v) => {
                if  f(v, &pivot) {
                    &arr.swap(i, j);
                    i += 1;
                }
            },

            _ => panic!("Array index for j out of bounds."),
        }
    }

    &arr.swap(i, high);
    i
}

/* Usage example */
fn main() {
    let mut arr: Vec<i32> = vec![3, 2, 1, 2, 2, 3, 5, 6];
    let len = arr.len();
    quick_sort::<i32>(&mut arr, 1, len - 1);


    println!("{:?}", arr);

}
