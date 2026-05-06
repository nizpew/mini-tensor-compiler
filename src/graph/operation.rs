use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub op_type: OpType,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpType {
    MatMul,
    ReLU,
    Add,
    Softmax,
    FusedLinearReLU,
}

impl Operation {
    pub fn new(name: String, op_type: OpType) -> Self {
        Self {
            name,
            op_type,
            inputs: Vec::new(),
            outputs: Vec::new(),
            attributes: HashMap::new(),
        }
    }
    
    pub fn with_inputs(mut self, inputs: Vec<String>) -> Self {
        self.inputs = inputs;
        self
    }
    
    pub fn with_outputs(mut self, outputs: Vec<String>) -> Self {
        self.outputs = outputs;
        self
    }
    
    pub fn add_input(&mut self, input: String) {
        self.inputs.push(input);
    }
    
    pub fn add_output(&mut self, output: String) {
        self.outputs.push(output);
    }
}

impl std::fmt::Display for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpType::MatMul => write!(f, "MatMul"),
            OpType::ReLU => write!(f, "ReLU"),
            OpType::Add => write!(f, "Add"),
            OpType::Softmax => write!(f, "Softmax"),
            OpType::FusedLinearReLU => write!(f, "FusedLinearReLU"),
        }
    }
}
