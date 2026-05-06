use crate::graph::{Graph, Operation, OpType};
use crate::passes::Pass;

pub struct FusionPass;

impl Pass for FusionPass {
    fn run(&self, graph: &mut Graph) {
        let operations = graph.operations_mut();
        let mut i = 0;
        
        while i < operations.len() {
            if i + 2 < operations.len() {
                let op1 = &operations[i];
                let op2 = &operations[i + 1];
                let op3 = &operations[i + 2];
                
                // Check for MatMul -> Add -> ReLU pattern
                if matches!(op1.op_type, OpType::MatMul) 
                    && matches!(op2.op_type, OpType::Add) 
                    && matches!(op3.op_type, OpType::ReLU) {
                    
                    // Create fused operation
                    let fused_name = format!("fused_linear_relu_{}", i);
                    let mut fused_op = Operation::new(fused_name, OpType::FusedLinearReLU);
                    
                    // Take inputs from first operation
                    fused_op.inputs = op1.inputs.clone();
                    
                    // Take outputs from last operation
                    fused_op.outputs = op3.outputs.clone();
                    
                    // Remove the three operations and insert the fused one
                    operations.splice(i..i+3, vec![fused_op]);
                    
                    println!("    ✨ Fused: MatMul -> Add -> ReLU -> FusedLinearReLU");
                    continue; // Don't increment i since we modified the vector
                }
            }
            i += 1;
        }
    }
    
    fn name(&self) -> &'static str {
        "FusionPass"
    }
}
