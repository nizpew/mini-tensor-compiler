mod graph;
mod passes;
mod runtime;
mod optimizer;
mod parser;
mod benchmark;
mod utils;

use graph::Graph;
use utils::logger;

fn main() {
    logger::init();
    
    println!("🚀 Mini Tensor Compiler");
    println!("========================");
    
    let mut graph = Graph::new();
    
    graph.add_op("MatMul");
    graph.add_op("Add");
    graph.add_op("ReLU");
    
    graph.print();
    
    println!("\n✅ Graph created successfully!");
}
