use std::io::Write;

/*
* returns index of the smallest element in vector.
* */
fn find_smallest<T>(array: &Vec<T>) -> usize
where
    T: std::cmp::PartialOrd
{
    let mut smallest_index: usize = 0;

    for i in 1..array.len() {
        if array[i] < array[smallest_index] {
            smallest_index = i;
        }
    }

    return smallest_index
}

/*
* return index of the biggest element in vector.
* */
fn find_max<T>(array: &Vec<T>) -> usize
where
    T: std::cmp::PartialOrd
{
    let mut max_index: usize = 0;

    for i in 1..array.len() {
        if array[i] < array[max_index] {
            max_index = i;
        }
    }

    return max_index;
}

/*
*  returns sorted vector (O(n * n))
* */
fn selection_sort<T>(arr: &mut Vec<T>) -> Vec<T>
where
    T: std::cmp::PartialOrd
{
    let mut result_arr: Vec<T> = Vec::new();

    for _ in 0..arr.len() {
        let smallest_index = find_smallest(&arr);
        result_arr.push(arr.remove(smallest_index));
    }

    return result_arr;
}

/*
* example of usage.
* */
fn main() {
    print!("Input a vector: ");
    std::io::stdout().flush().expect("something went wrong");

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string)
        .expect("Failed to read line.");

    let mut vector: Vec<f64> = Vec::with_capacity(10);
    for word in input_string.trim().split_whitespace() {
        let number: f64 = word.parse().expect("expected number.");
        vector.push(number);
    }

    let sorted_vector = selection_sort(&mut vector);

    println!("Sorted array: ");
    for item in sorted_vector {
        print!("{item} ");
        std::io::stdout().flush().expect("something went wrong");
    }

    println!();
}
