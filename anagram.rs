impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut db = std::collections::HashMap::<char, i32>::with_capacity(26);
        for (x, y) in s.chars().zip(t.chars()) {
            *db.entry(x).or_default() += 1;
            *db.entry(y).or_default() -= 1;
        }
    
        db.into_values().all(|count| count == 0)
    }
}

