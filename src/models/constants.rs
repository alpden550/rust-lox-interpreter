#[allow(dead_code)]
pub enum ExitCode {
    Success = 0,
    IncorrectCommand = 64,
    DataError = 65,
    NoInputFile = 66,
    RuntimeError = 70,
}
