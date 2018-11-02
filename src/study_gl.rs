use sample;
use permutohedron::Heap;
const DATA_ARRAY_LEN: usize = 14;


pub fn gen_sq(g1: &Vec<i32>, g2: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let zero: i32 = 0;

    // Row 1
    v.push(g1[0] + g2[0]);
    v.push(g1[1] + g2[1]);
    v.push(g1[2] + g2[2]);
    v.push(g1[3] + g2[3]);
    v.push(g1[4] + g2[4]);


    // Row 2
    v.push(g1[2] + g2[3]);
    v.push(g1[3] + g2[4]);
    v.push(g1[4] + g2[0]);
    v.push(g1[0] + g2[1]);
    v.push(g1[1] + g2[2]);

    // Row 3
    v.push(g1[4] + g2[1]);
    v.push(g1[0] + g2[2]);
    v.push(g1[1] + g2[3]);
    v.push(g1[2] + g2[4]);
    v.push(g1[3] + g2[0]);

    // Row 4
    v.push(g1[1] + g2[4]);
    v.push(g1[2] + g2[0]);
    v.push(g1[3] + g2[1]);
    v.push(g1[4] + g2[2]);
    v.push(g1[0] + g2[3]);

    // Row 5
    v.push(g1[3] + g2[2]);
    v.push(g1[4] + g2[3]);
    v.push(g1[0] + g2[4]);
    v.push(g1[1] + g2[0]);
    v.push(g1[2] + g2[1]);

    // Check that all the integers are unique.
    let mut sq_set = v.to_vec();
    sq_set.sort();
    sq_set.dedup();

    if sq_set.contains(&zero) {
        return vec![-1]
    }

    if sq_set.len() == 25 {
        v
    } else {
        vec![-1]
    }
}

pub fn find_best_perm(starter_set: &mut Vec<i32>) {
        let all_perms = Heap::new(starter_set);
        let mut array: [i64; DATA_ARRAY_LEN] = [0; DATA_ARRAY_LEN];

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



            if add_count != 12 { 
                array[0] += 1;
            } else {
                array[mult_count as usize] += 1;
            }

        }


        println!("results: {:?}", array);
}