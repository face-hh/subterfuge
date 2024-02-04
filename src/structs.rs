#[derive(Debug)]
pub struct SaveFile {
    pub(crate) money: i64,
    pub(crate) current_problem: i64,
    pub(crate) bp_xp: i64,
    pub(crate) bp_tier: i64,
    pub(crate) premium: bool,

    pub(crate) features: Vec<Feature>,
    pub(crate) problems: Vec<Problem>,
    pub(crate) battlepass: Vec<BattlepassTier>,
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

#[derive(Debug, Clone)]
pub struct BattlepassTier {
    pub(crate) index: String,
    pub(crate) _type: String,
    pub(crate) amount: i64,
    pub(crate) p: bool // premium
}