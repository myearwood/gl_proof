mod sample;
mod acc;
mod study_gl;
mod etl;
mod positionality;

extern crate permutohedron;
const DATA_ARRAY_LEN: usize = 13;

fn main() {

    let mut best_candidate = vec![-30, -23, 0, 30, 76, 78, 80, 95, 100, 114];
    let max_count = study_gl::max_perm_count(&mut best_candidate);
    println!("Max count: {:?}", max_count);


    let best_positions = positionality::best_positions(&mut best_candidate, max_count);

    println!("number of positons: {:?}", best_positions.len());
    let pairs_results = positionality::get_pairs_info(&best_positions);
    // let dedupped_results = positionality::dedup_pairs(&pairs_results);

    for (pair, bools) in pairs_results {
        println!("{}: {:?}", pair, bools);
    }


    println!("{:?}", best_candidate);
}
