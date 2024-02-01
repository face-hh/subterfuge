#[derive(Debug)]
pub struct SaveFile {
    pub(crate) money: i64,
    pub(crate) features: Vec<Feature>,
}

#[derive(Debug)]
pub struct Feature {
    pub(crate) item: String,
    pub(crate) cost: i64,
    pub(crate) unlocked: bool,
    pub(crate) regex: String,
}