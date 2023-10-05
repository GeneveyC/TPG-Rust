//! # Learner
//! Crate that permit to execute a learner of the Tangled Program Graph (TPG).
//! A team has multiple learners, each learner has a program which is executed to produce the bid value for this learner's action.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use crate::{flip, Brain, LearnerParams, Program};

#[derive(Clone, Debug)]
/// Structure that represetn a Learner.
pub struct Learner {
    /// The id of the learner.
    id: i32,
    /// The program of the learner.
    program: Program,
    /// The register of the learner.
    registers: Vec<i32>,
    /// The id of action observed by the learner.
    action: usize,
    /// The link with the team.
    in_teams: Vec<usize>,
}

impl PartialEq for Learner {
    /// Function to compare two learner between them.
    fn eq(&self, other: &Self) -> bool {
        self.program == other.program
    }
}

impl Learner {
    /// Constructor of the learner
    pub fn new(
        init_params: &mut LearnerParams,
        program: Program,
        action: usize,
        num_register: usize,
    ) -> Self {
        let id: i32 = init_params.get_new_id_learner();
        let registers: Vec<i32> = vec![0; num_register];
        let in_teams: Vec<usize> = Vec::new();
        Learner {
            id,
            program,
            registers,
            action,
            in_teams,
        }
    }

    /// Function to get the len of register
    pub fn get_len_register(&self) -> usize {
        self.registers.len()
    }

    /// Function to get action
    pub fn get_idx_action(&self) -> usize {
        self.action
    }

    /// Function to get the program
    pub fn get_program(&self) -> Program {
        self.program.clone()
    }

    /// Function to reset the register
    pub fn zero_registers(&mut self) {
        let size_registers: i32 = self.registers.len().try_into().unwrap();
        let regs: Vec<i32> = vec![0, size_registers];
        self.registers = regs;
    }

    /// Function to set teams to learner
    pub fn set_in_teams(&mut self, team_idx: usize) {
        self.in_teams.push(team_idx);
    }

    /// Function to get the number of teams referencing
    pub fn num_teams_referencing(&self) -> i32 {
        self.in_teams.len().try_into().unwrap()
    }

    /// Function to get the id of the learner
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Function to get the action or the team of the learner
    pub fn get_action_team(&self, brain: &Brain) -> Option<usize> {
        brain.actions[self.action].get_action_team()
    }

    /// Function to get the action with the current state
    pub fn get_action(&self, brain: &Brain, state: &Vec<i32>, visited: &mut Vec<i32>) -> i32 {
        brain.actions[self.action].get_action(brain, state, visited)
    }

    /// Function to get the bid of the learner
    pub fn bid(&mut self, state: &Vec<i32>) -> i32 {
        self.program.execute(state, &mut self.registers);
        // println!("Learner - Bid {}", self.registers[0]);
        self.registers[0]
    }

    /// Function to return if the action is atomic
    pub fn is_action_atomic(&self, brain: &Brain) -> bool {
        brain.actions[self.action].is_atomic()
    }

    /// Function to mutate the learner
    pub fn mutate(
        &mut self,
        brain: &Brain,
        mutate_params: &LearnerParams,
        parent_team: usize,
        teams: &Vec<usize>,
        p_action_atom: f64,
    ) {
        let mut changed: bool = false;

        // Get the list of action in the brain
        let mut list_actions = brain.actions.to_vec();

        // Get the action mutable from the list of action in the brain.
        let action_mutate = list_actions.get_mut(self.action).unwrap();

        while !changed {
            if flip(mutate_params.p_prog_mut) {
                changed = true;
                self.program.mutate(&mutate_params.program);
            }

            if flip(mutate_params.p_act_mut) {
                changed = true;
                action_mutate.mutate(brain, parent_team, teams, p_action_atom, self.id);
            }
        }
    }
}
