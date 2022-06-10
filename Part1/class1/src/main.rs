use std::io;
use std::{fs::File, io::Read};
fn main() {
    /* Exp 1:  check is sub array of array */
    let first_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let second_arr = [6, 8, 10];

    let result = is_sub_array(&second_arr, &first_arr);

    println!("result: {:#?}", result);

    /* -------------- Done ----------------- */

    /* Exp 2:  Find Word   */

    // read file
    let mut file = File::open("src/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // print file contents
    // println!("{}", contents);

    println!("Please input your string check.");

    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    // remove enter from input value
    let len = input_value.len();
    input_value.truncate(len - 1);

    // print input value
    println!("You input_value: {}", input_value);

    find_word(&contents, &input_value);

    /* -------------- Done ----------------- */
}

// function check is sub array of array
fn is_sub_array(sub_array: &[i32], array: &[i32]) -> bool {
    let mut index_array = 0;
    let mut index_sub_array = 0;
    let mut result = false;

    while index_array < array.len() && index_sub_array < sub_array.len() {
        if array[index_array] == sub_array[index_sub_array] {
            index_sub_array += 1;
        }
        index_array += 1;
    }

    if index_sub_array == sub_array.len() {
        result = true;
    }

    result
}

// function find word count in a string
fn find_word(string: &str, word: &str) {
    let vec_result: Vec<_> = string.match_indices(word).collect();
    // print vec_result length
    println!("{} occurrences", vec_result.len());
    // print vec_result content
    println!("occurrences location : {:#?}", vec_result);
}
