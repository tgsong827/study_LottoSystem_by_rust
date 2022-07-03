pub struct LottoPaper {
    numbers: Vec<i32>,
}

impl LottoPaper {
    pub fn new() -> LottoPaper {
        LottoPaper {
            numbers: Vec::new(),
        }
    }
}