mod sample;
mod acc;
mod study_gl;
mod etl;
mod positionality;

extern crate permutohedron;
const DATA_ARRAY_LEN: usize = 13;

fn main() {

    let mut best_candidate = vec![30, 23, 0, 30, 76, 78, 80, 95, 100, 114];
    println!("{:?}", best_candidate);
}
