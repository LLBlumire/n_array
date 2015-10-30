pub fn index_to_position(dimensions: usize, magnitudes: &Vec<usize>, index: &[usize]) -> usize {
    if index.len() != dimensions {
        panic!("Attempted to index NArray of dimension {} but {} dimensions supplied",
               dimensions,
               index.len());
    }
    let mut target_index = 0;
    for ((i, max), index) in magnitudes.iter().enumerate().zip(index.into_iter()) {
        if index >= max {
            panic!("Index out of bounds, attempted to index NArray dimension {} with {} but
                dimension only has magnitude {}", i, index, max);
        }
        target_index += index * magnitudes.iter().take(i).fold(1, |acc, &item| acc*item);
    }
    println!("TI {}", target_index);
    target_index
}

pub fn position_to_index(mag: &[usize], index: usize) -> Vec<usize> {
    let mag = mag.to_vec();
    let mut eval: Vec<usize> = Vec::new();
    let mut index_buffer = index;
    for x in (0..mag.len()).rev() {
        let sum_parts: usize = mag.clone().iter().take(x).fold(1, |acc, &item| acc*item);
        let inter: usize = index_buffer / sum_parts;
        eval.push( inter );
        index_buffer -= sum_parts * inter;
    }
    eval.iter().rev().map(|c| *c).collect::<Vec<usize>>()
}
