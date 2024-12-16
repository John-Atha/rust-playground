use std::collections::HashMap;

/**
 * <b>Exercise</b>
 * Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
 * <b>Notes on the Solution</b>
 * Obviously it is less optimal to create two separate functions and parse/iterate the vector separately on each one
 * But let's keep it simple for this playground stuff
 */
fn main() {
    let mut v: Vec<i32> = vec![11, 74, 2, 6, 38, 4, 5, 6, 104, 7, 8, 24, 89, 10];
    let median = get_median(&mut v);
    let (max_num, max_counter) = get_mode(&v);

    println!("The median is: {median}");
    println!("The number {max_num} was seen {max_counter} times!");
}

fn get_mode(v: &Vec<i32>) -> (i32, u32) {
    let mut count: HashMap<i32, u32> = HashMap::new();
    for i in v {
        let c = count.entry(*i).or_insert(0);
        *c += 1;
    }
    let mut max_num = 0;
    let mut max_counter = 0;
    for (num, counter) in count.iter() {
        if count[num] > max_counter {
            max_num = *num;
            max_counter = *counter;
        }
    }

    (max_num, max_counter)
}

/**
 * Interesting task to implement this function without mutating the provided vector
 * If we try to copy it with v\[..\], we get an error that <i>all local variables must have a statically known size</i>
 */
fn get_median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    v[v.len() / 2]
}
