mod sample;
mod acc;
mod study_gl;
mod etl;
extern crate permutohedron;
const DATA_ARRAY_LEN: usize = 13;

fn k_formula(k: i32) -> i32 {
    let mut items = vec![ k-30, k-23, k, k+30, k+76, k+78, k+80, k+95, k+100, k+114];
    println!("set: {:?}", items);
    study_gl::max_perm_count(&mut items)
}


fn main() {

    for k in 0..30 {
        let count = k_formula(k);
        // println!("K: {:?}", k);
        println!("count: {:?}", count);    
    }


    // max == 10 magic products
    // let mut best_pair = vec![-30, -23, 0, 30, 76, 78, 80, 95, 100, 114];

    // let mut best_pair = vec![];

    // study_gl::find_perm_distribution(&mut best_pair);
    // println!("Hello, world!");
}
