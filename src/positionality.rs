use study_gl::gen_sq;
use permutohedron::Heap;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};


use sample;

fn find_num_positon(array: &Vec<i32>, num: i32, len: usize) -> i32 {
    for i in 0..len {
        if array[i] == num {
            return i as i32
        }
    }

    return -1 as i32
}



fn get_pos_pairs(array: &Vec<i32>) -> Vec<(i32, i32, bool)> {
    let mut results: Vec<(i32, i32, bool)> = Vec::new();

    for i in 0..10 {
        let current_num = array[i];

        // get the current group
        if i < 5 {
            // current group is 1
            for x in 0..5 {
                if x != i {
                    let res_pair = (current_num, array[x], true);
                    results.push(res_pair);
                }
            }

            for x in 5..10 {
                let res_pair = (current_num, array[x], false);
                results.push(res_pair);
            }
        
        } else {
            // current group is 2
            for x in 0..5 {
                let res_pair = (current_num, array[x], false);
                results.push(res_pair)
            }

            for x in 5..10 {
                if x != i {
                    let res_pair = (current_num, array[x], true);
                    results.push(res_pair);                   
                }

            }            

        }
    }

    return results;
}


pub fn get_pairs_info(best_positions: &Vec<Vec<i32>>) -> HashMap<String, (i32, i32)> {
    let mut results: HashMap<String, (i32, i32)> = HashMap::new();

    for pos in best_positions {
        let pos_pairs = get_pos_pairs(&pos);

        for pair in &pos_pairs {
            let (num1, num2, paired) = pair;

            let new_pair = match*paired {
                true =>  (1, 0),
                false => (0, 1),
            };
            

            let key = format!("{}_{}", num1, num2);

            results.entry(key)
               .and_modify(|bool_pair| { 
                    let (true_count, false_count) = bool_pair;
                    if *paired {
                        *true_count += 1;
                    } else {
                        *false_count += 1;
                    } 
               })
               .or_insert(new_pair);
        }
    }

    results
}

fn get_dedupped_pair(short_pair: String) -> String {
    let nums_split = short_pair.split("_");
    let nums: Vec<&str> = nums_split.collect();

    let num1: i32 = nums[0].parse().unwrap();
    let num2: i32 = nums[1].parse().unwrap();

    if num1 > num2 {
        return format!("{}_{}", num1, num2)
    } else {
        return format!("{}_{}", num2, num1)
    }
}


pub fn dedup_pairs(raw_results: &HashMap<String, (i32, i32)>) -> HashMap<String, (i32, i32)> {
    let mut results: HashMap<String, (i32, i32)> = HashMap::new();

    for (pair, bools) in raw_results {
        let dedupped_pair = get_dedupped_pair(pair.to_string());
        results.entry(dedupped_pair).or_insert(*bools);
    }

    results
}

pub fn new_position(orginal_position: &Vec<i32>, new_position: &Vec<i32>)  -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    let len: usize = orginal_position.len();
    for i in 0..len {
        let num = orginal_position[i];
        let new_pos = find_num_positon(new_position, num, len);
        results.push(new_pos)
    }

    results
}


pub fn best_positions(array: &mut Vec<i32>, max_count: i32) -> Vec<Vec<i32>> {
        let ref_array = array.clone();
        let all_perms = Heap::new(array);
        let mut results: Vec<Vec<i32>> = Vec::new();

        for perm in all_perms {
            let r = perm.clone();

            let mut g1: Vec<i32> = vec![];
            let mut g2: Vec<i32> = vec![];

            for i in 0..5 {
                g1.push(r[i])
            }

            for i in 5..10 {
                g2.push(r[i])
            }


            let rand_sq = gen_sq(&g1, &g2);
            let add_count = sample::add_magic(&rand_sq, 5 as i32);
            let mult_count = sample::mult_magic(&rand_sq, 5 as i32);

            if add_count == 12 {
                if  mult_count == max_count {
                    let new_pos = new_position(&ref_array, &r);
                    results.push(new_pos);
                }
            }
        }

        results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_postion_missing_nums() {
        let original = vec![1,2,3,4, 5, 6, 7, 8];
        let new = vec![15,7,6,5,4,2,3,1];

        let results = new_position(&original, &new);
        let expected_result = vec![7,5,6,4,3,2,1,-1];

        assert_eq!(expected_result, results);
    }

    #[test]
    fn test_new_postion() {
        let original = vec![1,2,3,4];
        let new = vec![4,2,3,1];

        let results = new_position(&original, &new);
        let expected_result = vec![3,1,2,0];

        assert_eq!(expected_result, results);        
    }



    #[test]
    fn test_get_pos_pairs() {
        let items: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
        let pos_pairs = get_pos_pairs(&items);

        println!("{:?}", pos_pairs);
        assert_eq!(pos_pairs.len(), 90);
    }

    #[test]
    fn test_get_dedupped_pair() {
        let p1 = String::from("6_2");
        let p2 = String::from("2_6");

        let e1 = get_dedupped_pair(p1);
        let e2 = get_dedupped_pair(p2);

        assert_eq!(e1, e2);
    }

}

