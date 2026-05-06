use mini_tensor_compiler::graph::Graph;
use mini_tensor_compiler::passes::{FusionPass, Pass};

#[test]
fn test_fusion_pass() {
    let mut graph = Graph::new();
    graph.add_op("MatMul");
    graph.add_op("Add");
    graph.add_op("ReLU");
    
    assert_eq!(graph.operations().len(), 3);
    
    let fusion_pass = FusionPass;
    fusion_pass.run(&mut graph);
    
    // After fusion, should have 1 operation instead of 3
    assert_eq!(graph.operations().len(), 1);
    assert!(matches!(graph.operations()[0].op_type, mini_tensor_compiler::graph::OpType::FusedLinearReLU));
}
