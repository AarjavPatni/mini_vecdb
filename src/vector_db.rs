use std::{collections::HashMap, error::Error};

use nalgebra::SVector;

type Embedding = SVector<f32, 1024>;
type VectorDB = HashMap<String, Embedding>;

fn distance(e1: &Embedding, e2: &Embedding) -> f32 {
    e1.dot(e2) / (e1.norm() * e2.norm())
}

pub fn search(db: VectorDB, query: Embedding, k: usize) -> Result<Vec<Embedding>, String> {
    // goal: find the top-k results.
    // exit if db len < 10
    // get first 10 ele of db, check distance with query, add to array as (embedding, distance)
    // sort array using distance element
    // for every next ele, insert before the first element compared to which the distance is
    // greater

    if db.len() < k {
        return Err(format!("Size of the database cannot be less than {}", k));
    }

    let mut top_k: [Embedding; 10];
    let keys = db.keys();

    todo!()
}

pub fn store(db: &mut VectorDB, url: String, embedding: Embedding) -> Result<String, String> {
    // goal: push embedding into db
    // the insert fn returns None is the key doesn't exist, and the old value if it does

    match db.insert(url, embedding) {
        None => Ok("Key inserted".to_string()),
        Some(i) => Ok(format!("Key replaced. Previous value: {}", i).to_string()),
    }
}

