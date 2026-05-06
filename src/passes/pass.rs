use crate::graph::Graph;

pub trait Pass {
    fn run(&self, graph: &mut Graph);
    fn name(&self) -> &'static str;
}

pub struct PassManager {
    passes: Vec<Box<dyn Pass>>,
}

impl PassManager {
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
        }
    }
    
    pub fn add_pass(&mut self, pass: Box<dyn Pass>) {
        self.passes.push(pass);
    }
    
    pub fn run_all(&mut self, graph: &mut Graph) {
        println!("\n🔄 Running optimization passes...");
        
        for pass in &self.passes {
            println!("  Running: {}", pass.name());
            pass.run(graph);
        }
        
        println!("  ✅ All passes completed");
    }
}
