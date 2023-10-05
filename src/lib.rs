mod action;
mod agent;
mod brain;
mod learner;
mod param;
mod program;
mod team;
mod trainer;

pub use action::Action;
pub use agent::Agent;
pub use brain::Brain;
pub use learner::Learner;
pub use param::{Params,
    ActionParams, LearnerParams, ProgramParams, TeamParams,
    TrainerParams,
};
pub use program::Program;
pub use team::Team;
pub use trainer::Trainer;

use rand::Rng;

/// Function to determine if the proba is apply
pub fn flip(proba: f64) -> bool {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    y < proba
}
