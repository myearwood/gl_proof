use study_gl::gen_sq;
use permutohedron::Heap;
use sample;

fn find_num_positon(array: &Vec<i32>, num: i32, len: usize) -> i32 {
    for i in 0..len {
        if array[i] == num {
            return i as i32
        }
    }

    return -1 as i32
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

}

