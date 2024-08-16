#[derive(Debug, Clone)]
pub struct LVar {
    name: String,
    pub offset: usize,
}

impl LVar {
    pub fn new(name: String, offset: usize) -> LVar {
        LVar { name, offset }
    }
}
