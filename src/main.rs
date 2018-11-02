mod sample;
mod acc;
mod study_gl;
mod etl;
extern crate permutohedron;


// fn k_formula(k: i32) {

//     let items = vec![ k-30, k-23, k, k+30, k+76, k+78, ];
// }

fn main() {

    // max == 10 magic products
    // let mut best_pair = vec![-30, -23, 0, 30, 76, 78, 80, 95, 100, 114];


    let mut best_pair = vec![];

    study_gl::find_perm_distribution(&mut best_pair);
    println!("Hello, world!");
}
