use crate::graph::operation::{Operation, OpType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph {
    operations: Vec<Operation>,
    tensors: HashMap<String, crate::graph::tensor::Tensor>,
    operation_counter: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            tensors: HashMap::new(),
            operation_counter: 0,
        }
    }
    
    pub fn add_op(&mut self, op_type_str: &str) -> &mut Self {
        let op_type = match op_type_str {
            "MatMul" => OpType::MatMul,
            "ReLU" => OpType::ReLU,
            "Add" => OpType::Add,
            "Softmax" => OpType::Softmax,
            "FusedLinearReLU" => OpType::FusedLinearReLU,
            _ => panic!("Unsupported operation type: {}", op_type_str),
        };
        
        let op_name = format!("{}_{}", op_type_str.to_lowercase(), self.operation_counter);
        let operation = Operation::new(op_name.clone(), op_type);
        
        self.operations.push(operation);
        self.operation_counter += 1;
        
        // Connect operations sequentially if there are multiple
        if self.operations.len() > 1 {
            let len = self.operations.len();
            let prev_output = self.operations[len - 2].outputs.first().cloned();
            let current_op = &mut self.operations[len - 1];
            
            if let Some(output) = prev_output {
                current_op.add_input(output);
            }
            
            let output_name = format!("{}_output", current_op.name);
            current_op.add_output(output_name);
        } else {
            // First operation
            let current_op = &mut self.operations[0];
            let output_name = format!("{}_output", current_op.name);
            current_op.add_output(output_name);
        }
        
        self
    }
    
    pub fn add_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }
    
    pub fn connect(&mut self, from_op: &str, to_op: &str) {
        let from_idx = self.operations.iter().position(|op| op.name == from_op);
        let to_idx = self.operations.iter().position(|op| op.name == to_op);
        
        if let (Some(from_idx), Some(to_idx)) = (from_idx, to_idx) {
            if !self.operations[from_idx].outputs.is_empty() {
                let output_name = self.operations[from_idx].outputs[0].clone();
                self.operations[to_idx].add_input(output_name);
            }
        }
    }
    
    pub fn print(&self) {
        println!("\n📊 Graph Structure:");
        println!("==================");
        
        for (i, op) in self.operations.iter().enumerate() {
            if i == 0 {
                print!("{}", op.op_type);
            } else {
                print!(" -> {}", op.op_type);
            }
        }
        println!();
        
        println!("\n📋 Detailed Operations:");
        for op in &self.operations {
            println!("  {}: {}", op.name, op.op_type);
            if !op.inputs.is_empty() {
                println!("    Inputs: {:?}", op.inputs);
            }
            if !op.outputs.is_empty() {
                println!("    Outputs: {:?}", op.outputs);
            }
        }
    }
    
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }
    
    pub fn operations_mut(&mut self) -> &mut Vec<Operation> {
        &mut self.operations
    }
}
