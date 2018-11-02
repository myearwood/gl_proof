mod sample;
mod acc;
mod study_gl;
extern crate permutohedron;

fn main() {
    let mut best_pair = vec![-30, -23, 0, 30, 76, 78, 80, 95, 100, 114];

    study_gl::find_best_perm(&mut best_pair);
    println!("Hello, world!");
}
