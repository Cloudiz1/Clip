#[derive(Debug)]
pub enum Error {
    UnknownArgument(String),
    ExpectedParameter {
        argument: String,
        parameter: String,
    }
}
