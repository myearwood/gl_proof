
fn find_num_positon(array: &Vec<i32>, num: i32, len: usize) -> i32 {
    for i in 0..len {
        if array[i] == num {
            return i as i32
        }
    }

    return -1 as i32
}


fn new_position(orginal_position: Vec<i32>, new_position: Vec<i32>)  -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    let len: usize = orginal_position.len();
    for i in 0..len {
        let num = orginal_position[i];
        let new_pos = find_num_positon(&new_position, num, len);
        results.push(new_pos)
    }

    results
}