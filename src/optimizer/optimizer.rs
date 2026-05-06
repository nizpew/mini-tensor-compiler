use crate::graph::Graph;
use crate::passes::{PassManager, FusionPass, ConstantFoldingPass, DeadCodeEliminationPass};

pub struct Optimizer {
    pass_manager: PassManager,
}

impl Optimizer {
    pub fn new() -> Self {
        let mut pass_manager = PassManager::new();
        
        // Add optimization passes in order
        pass_manager.add_pass(Box::new(ConstantFoldingPass));
        pass_manager.add_pass(Box::new(FusionPass));
        pass_manager.add_pass(Box::new(DeadCodeEliminationPass));
        
        Self { pass_manager }
    }
    
    pub fn run_all_passes(&mut self, graph: &mut Graph) {
        self.pass_manager.run_all(graph);
    }
    
    pub fn add_pass(&mut self, pass: Box<dyn crate::passes::Pass>) {
        self.pass_manager.add_pass(pass);
    }
}
