pub fn find<T>(vec: &Vec<T>, item: &T) -> Option<usize>
    where T: Ord + Clone {
    let mut new_vec = vec.clone();
    new_vec.sort();

    let mut from: usize = 0;
    let mut to: usize = new_vec.len();

    loop {
        let seed = (from + to) / 2;
        if &new_vec[seed] == item {
            return Some(seed);
        }
        if &new_vec[seed] < item {
            to = seed - 1;
        } else {
            from = seed + 1;
        }
        if from >= to {
            return None;
        }
    } // loop
}
