#[derive(Debug)]
pub struct ProgramIsland {
    pub nests: Vec<Nest>,
    pub sea_flow: Flow
}

#[derive(Debug)]
pub enum Nest {
    Green(Vec<String>),
}

#[derive(Debug)]
pub struct Flow(pub Vec<Value>);

#[derive(Debug)]
pub enum Value {
    Name(String),
    String(String),
    Number(i32),
    InternalFlow(Vec<Value>),
    Trigger,
}
