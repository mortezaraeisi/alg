use search;

fn main() {
    let collection = vec![10, 20, 30, 15, 25, 45, 95, 100, 85, 10, 20, 65, 75, 70, 30];
    let collection_inv = vec![1, 2, 3, 4];

    println!("usize::BIT is {}", usize::BITS);

    println!("Start linear");
    linear(&collection, &collection_inv);

    println!("Start binary");
    binary(&collection, &collection_inv);
}

fn linear<T>(coll: &Vec<T>, inv: &[T])
    where T: Ord {
    assert!(search::linear::find(coll, &coll.last().unwrap()).is_some(), "Should find [linear]");
    assert!(search::linear::find(coll, &inv.last().unwrap()).is_none(), "Should not find [linear]");
}

fn binary<T>(coll: &Vec<T>, inv: &[T])
    where T: Ord + Clone {
    assert!(search::binary::find(coll, &coll.last().unwrap()).is_some(), "Should find [binary]");
    assert!(search::binary::find(coll, &inv.last().unwrap()).is_none(), "Should not find [binary]");
}