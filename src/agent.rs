//! # Agent
//! Crate that permit to represent the agent of the Tangled Program Graph (TPG).

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use crate::brain::Brain;

/// Structure to represent the agent
pub struct Agent {
    /// The team associate to the agent
    team: usize,
}

impl Agent {
    /// Constructor of the agent
    pub fn new(team: usize) -> Self {
        Agent {
            team,
        }
    }

    /// Function to get the index of team
    pub fn get_idx_teams(&self) -> usize {
        self.team
    }

    /// Function to act the agent
    pub fn act(&mut self, brain: &mut Brain, state: &Vec<i32>) -> i32 {
        let mut visited: Vec<i32> = Vec::new();
        let mut list_teams = brain.teams.to_vec();
        let team = list_teams.get_mut(self.team).unwrap();
        team.act(brain, state, &mut visited)
    }

    /// Function to set the reward into the team
    pub fn reward(&mut self, brain: &mut Brain, score: i32, task: String) {
        let team = brain.teams.get_mut(self.team).unwrap();
        team.set_outcomes(task, score);
    }

    /// Function to check if the task is done
    pub fn task_done(&self, brain: &Brain, task: String) -> bool {
        let team = brain.teams.get(self.team).unwrap();
        team.task_done(task)
    }

    /// Function to reset the register to zero
    pub fn zero_registers(&mut self, brain: &Brain) {
        let mut list_teams = brain.teams.to_vec();
        let team = list_teams.get_mut(self.team).unwrap();
        team.zero_registers(brain);
    }
}
