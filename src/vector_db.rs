use std::{collections::HashMap, error::Error};

use nalgebra::SVector;

type Embedding = SVector<f32, 1024>;
type VectorDB = HashMap<String, Embedding>;

fn distance(e1: &Embedding, e2: &Embedding) -> f32 {
    e1.dot(e2) / (e1.norm() * e2.norm())
}

pub fn search(db: VectorDB, query: Embedding, k: usize) -> Result<Vec<Embedding>, String> {
    // goal: find the top-k results.
    todo!()
}

pub fn store(db: VectorDB, embedding: Embedding) -> Result<(), String> {
    // goal: push embedding into db
    todo!()
}

