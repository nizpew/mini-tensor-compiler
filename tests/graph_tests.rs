use mini_tensor_compiler::graph::Graph;
use mini_tensor_compiler::graph::OpType;

#[test]
fn test_graph_creation() {
    let mut graph = Graph::new();
    graph.add_op("MatMul");
    graph.add_op("ReLU");
    
    assert_eq!(graph.operations().len(), 2);
    assert!(matches!(graph.operations()[0].op_type, OpType::MatMul));
    assert!(matches!(graph.operations()[1].op_type, OpType::ReLU));
}

#[test]
fn test_graph_printing() {
    let mut graph = Graph::new();
    graph.add_op("Add");
    graph.add_op("Softmax");
    
    // Just test that printing doesn't panic
    graph.print();
}
