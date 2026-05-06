use std::time::Instant;

pub struct Profiler {
    start_time: Option<Instant>,
    measurements: Vec<(&'static str, std::time::Duration)>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            start_time: None,
            measurements: Vec::new(),
        }
    }
    
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        println!("⏱️  Profiling started");
    }
    
    pub fn measure(&mut self, name: &'static str) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            self.measurements.push((name, elapsed));
            println!("  📊 {}: {:?}", name, elapsed);
        }
    }
    
    pub fn finish(&self) {
        println!("\n📈 Profile Summary:");
        for (name, duration) in &self.measurements {
            println!("  {}: {:?}", name, duration);
        }
    }
}
