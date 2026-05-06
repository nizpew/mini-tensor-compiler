use mini_tensor_compiler::graph::Graph;

fn main() {
    println!("🚀 Simple Graph Example");
    println!("======================");
    
    let mut graph = Graph::new();
    
    graph.add_op("MatMul");
    graph.add_op("Add");
    graph.add_op("ReLU");
    
    graph.print();
}
