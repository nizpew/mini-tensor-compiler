use mini_tensor_compiler::graph::Graph;
use mini_tensor_compiler::optimizer::Optimizer;

fn main() {
    println!("🔥 Fusion Demo");
    println!("===============");
    
    let mut graph = Graph::new();
    
    // Create a graph that can be fused
    graph.add_op("MatMul");
    graph.add_op("Add");
    graph.add_op("ReLU");
    
    println!("\n📊 Before optimization:");
    graph.print();
    
    // Run optimization
    let mut optimizer = Optimizer::new();
    optimizer.run_all_passes(&mut graph);
    
    println!("\n📊 After optimization:");
    graph.print();
}
