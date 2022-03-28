struct Elapsed(&'static str, std::time::SystemTime);

impl Drop for Elapsed {
    fn drop(&mut self) {
        println!(
            "operation {} finished for {} ms",
            self.0,
            self.1.elapsed().unwrap_or_default().as_millis()
        );
    }
}

impl Elapsed {
    pub fn start(op: &'static str) -> Elapsed {
        let now = std::time::SystemTime::now();

        Elapsed(op, now)
    }
}

//let _exec_time = Elapsed::start("some_heavy_work_fn");