use std::io::Write;

/*
* returns index of first occurrence in vector. (O(log(n)))
* */
fn binary_search(array: &Vec<i32>, value: i32) -> i32 {
    let mut index_of_value = -1;
    let mut left = 0;
    let mut right = array.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;

        match value.cmp(&array[mid as usize]) {
            std::cmp::Ordering::Equal => {
                index_of_value = mid;
                right = mid - 1;
            },
            std::cmp::Ordering::Less => right = mid - 1,
            std::cmp::Ordering::Greater => left = mid + 1
        }
    }

    return index_of_value;
}

/*
* returns index of last occurrence in vector (O(log(n)))
* */
fn rbinary_search(array: &Vec<i32>, value: i32) -> i32 {
    let mut index_of_value = -1;
    let mut left = 0;
    let mut right = array.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;

        match value.cmp(&array[mid as usize]) {
            std::cmp::Ordering::Equal => {
                index_of_value = mid;
                left = mid + 1;
            },
            std::cmp::Ordering::Less => right = mid - 1,
            std::cmp::Ordering::Greater => left = mid + 1,
        }
    }

    return index_of_value;
}

fn delim_string(string: &str) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();

    for word in string.trim().split_whitespace() {
        let number: i32 = word.parse()
            .expect("expected number in vector");
        vector.push(number);
    }

    return vector;
}

/* example of usage. */
fn main() {
    let mut input_string = String::new();

    print!("input vector: ");
    std::io::stdout().flush().expect("wtf");
    std::io::stdin().read_line(&mut input_string)
        .expect("error while reading string");
    let vector = delim_string(&input_string);

    let mut input_string = String::new();
    print!("input a value you want to find: ");
    std::io::stdout().flush().expect("wtf");
    std::io::stdin().read_line(&mut input_string)
        .expect("error while reading string");
    let value: i32 = input_string.trim().parse().expect("expected number");

    println!("{}", rbinary_search(&vector, value));
}
