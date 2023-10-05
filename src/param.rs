//! # Param
//! Crate that permit to define the parameter of initializeation and mutation inside the Tangled Program Graph (TPG).

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use std::fs::File;
use serde::{Serialize, Deserialize};

/// Structure to represent all information for the init and mutation of program
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ProgramParams {
    /// The id counter of program.
    pub id_counter_program: i32,
    /// The len of the program.
    pub max_program_length: i32,
    /// The number of operation possible in the program.
    pub nb_operations: i32,
    /// The size of the input.
    pub input_size: i32,
    /// The number of destination possilbe in the program.
    pub nb_destinations: i32,
    /// The probability to delete the instruction.
    pub p_inst_del: f64,
    /// The probability to mutate the instruction.
    pub p_inst_mut: f64,
    /// The probability to swap the instruction.
    pub p_inst_swap: f64,
    /// The probability to add the instruction.
    pub p_inst_add: f64,
}

impl ProgramParams {
    /// Function to return a new id for the program.
    pub fn get_new_id_program(&mut self) -> i32 {
        let id_program = self.id_counter_program;
        self.id_counter_program += 1;
        id_program
    }
}

/// Structure to represent all information for the init and mutation of program.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct LearnerParams {
    /// The id counter of learner
    pub id_counter_learner: i32,
    /// The probability to mutate the program in the learner
    pub p_prog_mut: f64,
    /// The probability to mutate the action in the learner
    pub p_act_mut: f64,
    /// The list of mutation parameter for the program in the learner
    pub program: ProgramParams,
    /// The list of mutation parameter for the action in the learner
    pub action: ActionParams,
}

impl LearnerParams {
    /// Function to return a new id for the learner
    pub fn get_new_id_learner(&mut self) -> i32 {
        let id_learner: i32 = self.id_counter_learner;
        self.id_counter_learner += 1;
        id_learner
    }
}

/// Structure to represent all information for the init and mutation of action.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ActionParams {
    /// The id counter of action
    pub id_counter_action: i32,
}

impl ActionParams {
    /// Function to return a new id for the action
    pub fn get_new_id_action(&mut self) -> i32 {
        let id_action = self.id_counter_action;
        self.id_counter_action += 1;
        id_action
    }
}

/// Structure to represent all information for the init and mutation of the team.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct TeamParams {
    /// The id counter of team
    pub id_counter_team: i32,
    /// The probability to add learner in the team.
    pub p_lrn_add: f64,
    /// The probability to delete learner in the team.
    pub p_lrn_del: f64,
    /// The probability to mutate learner in the team.
    pub p_lrn_mut: f64,
    /// The probability to mutate the action.
    /// If no select then mutate the team associate to the action.
    pub p_act_atom: f64,
    /// The rampant gen to the team.
    pub rampant_gen: i32,
    /// The rampant min to the team.
    pub rampant_min: i32,
    /// THe rampant max to the team.
    pub rampant_max: i32,
    /// The probabiliy information about the learner
    pub learner: LearnerParams,
}

impl TeamParams {
    /// Function to return a new id for the team
    pub fn get_new_id_team(&mut self) -> i32 {
        let id_team = self.id_counter_team;
        self.id_counter_team += 1;
        id_team
    }
}

/// Structure to represent all information for the init and mutation of the trainer
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct TrainerParams {
    /// The id counter of the trainer
    pub id_counter_trainer: i32,
    /// The number of team inside the trainer
    pub max_learner_in_team: i32,
    /// The number of team in the population
    pub max_team_in_population: i32,
    /// The generation in the trainer
    pub generation: i32,
    /// The probability information about the team
    pub team: TeamParams,
}

impl TrainerParams {
    /// Constructor of the trainer mutate params
    pub fn new() -> Self {
        TrainerParams {
            id_counter_trainer: 0,
            max_learner_in_team: 5,
            max_team_in_population: 5,
            generation: 0,
            team: TeamParams::default(),
        }
    }
}

impl TrainerParams {
    /// Function to return a new id for the trainer
    pub fn get_new_id_trainer(&mut self) -> i32 {
        let id_trainer = self.id_counter_trainer;
        self.id_counter_trainer += 1;
        id_trainer
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Structure to represent the parameters
pub struct Params {
    /// The probability informations about the trainer
    pub trainer: TrainerParams
}

impl Params {
    /// Constructor of the Params
    pub fn new(filename_path: String) -> Self {
        let file = File::open(filename_path).unwrap();
        let params: Params = serde_json::from_reader(file).expect("error while reading or parsing");
        params
    }
}