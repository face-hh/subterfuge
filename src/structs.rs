#[derive(Debug)]
pub struct SaveFile {
    pub(crate) money: i64,
    pub(crate) current_problem: i64,

    pub(crate) features: Vec<Feature>,
    pub(crate) problems: Vec<Problem>,
}

#[derive(Debug)]
pub struct Feature {
    pub(crate) item: String,
    pub(crate) cost: i64,
    pub(crate) unlocked: bool,
    pub(crate) regex: String,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub(crate) index: i64,
    pub(crate) name: String,
    pub(crate) append: String,
    pub(crate) description: String,
    pub(crate) starting_code: String,
    pub(crate) money: i64
}