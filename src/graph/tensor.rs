use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor {
    pub name: String,
    pub shape: Vec<usize>,
    pub dtype: DataType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Float32,
    Float64,
    Int32,
    Int64,
    Bool,
}

impl Tensor {
    pub fn new(name: String, shape: Vec<usize>, dtype: DataType) -> Self {
        Self { name, shape, dtype }
    }
    
    pub fn size(&self) -> usize {
        self.shape.iter().product()
    }
}

impl std::fmt::Display for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.shape)
    }
}
