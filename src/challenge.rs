use std::time::Instant;

pub trait Challenge {
    fn run(&self);
    fn time_run(&self) {
        let start = Instant::now();
        self.run();
        let duration = start.elapsed();
        println!("Execution time: {:?}", duration);
    }
}
