pub mod grid;
pub mod solver;

type Coord = euclid::Vector2D<i32, euclid::UnknownUnit>;

#[derive(Debug)]
pub enum TwelfthError {
    FileProblem(String),
    InputDoesNotContainAnyStart,
    InputDoesNotContainAnyFinish,
    SolverCouldNotFindASolutionToTarget,
}

impl From<std::io::Error> for TwelfthError {
    fn from(e: std::io::Error) -> Self {
        Self::FileProblem(format!("{}", e))
    }
}
