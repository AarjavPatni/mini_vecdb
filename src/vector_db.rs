use std::{collections::HashMap};

use nalgebra::{SVector};

type Embedding = SVector<f32, 256>;
type VectorDB = HashMap<String, Embedding>;

fn similarity(e1: &Embedding, e2: &Embedding) -> f32 {
    e1.dot(e2) / (e1.norm() * e2.norm())
}

pub fn search(db: &VectorDB, query: Embedding, k: usize) -> Result<Vec<(Embedding, f32)>, String> {
    // goal: find the top-k results.
    // exit if db len < k
    // get first k ele of db, check distance with query, add to array as (embedding, distance)
    // sort array using distance element
    // for every next ele, insert before the first element compared to which the distance is
    // greater

    if db.len() < k {
        return Err(format!("Size of the database cannot be less than {}", k));
    }

    let mut top_k: Vec<(Embedding, f32)> = vec![];
    let keys_vec: Vec<String> = db.keys().cloned().collect();

    for i in 0..keys_vec.len() {
        let e1 = db.get(&keys_vec[i]).unwrap().clone();
        top_k.push((e1, similarity(&e1, &query)));
        top_k.sort_by(|a, b| a.1.total_cmp(&b.1));
        if top_k.len() > k {
            top_k.pop();
        }
    }

    Ok(top_k)
}

pub fn store(db: &mut VectorDB, url: String, embedding: Embedding) -> Result<String, String> {
    // goal: push embedding into db
    // the insert fn returns None is the key doesn't exist, and the old value if it does

    match db.insert(url, embedding) {
        None => Ok("Key inserted".to_string()),
        Some(i) => Ok(format!("Key replaced. Previous value: {}", i).to_string()),
    }
}

