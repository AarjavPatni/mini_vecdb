use std::error::Error;

type Embedding = [f32; 1024];

pub struct VectorDB {
    id: String,
    embedding: Embedding,
}

impl VectorDB {
    fn distance() -> f32 {
        todo!()
    }

    pub fn search(target: Embedding, k: usize) -> Result<Vec<Embedding>, String> {
        todo!()
    }

    pub fn store() -> Result<(), String> {
        todo!()
    }
}

