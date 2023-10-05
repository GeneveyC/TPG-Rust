//! # Action
//! Crate that permit to represent the action of the Tangled Program Graph (TPG).

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use crate::{flip, ActionParams, Brain};
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
/// Structure to represent the action.
pub struct Action {
    /// The id of the action
    id: i32,
    /// The action code.
    action_code: i32,
    /// The team link to the action.
    action_team: Option<usize>,
}

impl PartialEq for Action {
    /// Function to compare two action between them.
    fn eq(&self, other: &Self) -> bool {
        self.action_code == other.action_code
    }
}

impl Action {
    /// Constructor of the action.
    pub fn new(action_code: i32, init_params: &mut ActionParams) -> Self {
        let id = init_params.get_new_id_action();
        Action {
            id,
            action_code,
            action_team: None,
        }
    }

    /// Function to get the id of the action
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Function to determine if the action is atomic.
    /// The action is atomic if they are not associate to a team.
    pub fn is_atomic(&self) -> bool {
        self.action_team.is_none()
    }

    /// Function to get the team associate to the action
    pub fn get_action_team(&self) -> Option<usize> {
        self.action_team
    }

    /// Function to get the action
    /// If the action is atomic
    ///     |-> Then return the action code
    /// If the action is not atomic
    ///     |-> Execute the team associate to the team
    pub fn get_action(&self, brain: &Brain, state: &Vec<i32>, visited: &mut Vec<i32>) -> i32 {
        if let Some(idx_team) = self.action_team {
            brain.teams[idx_team].act(brain, state, visited)
        } else {
            self.action_code
        }
    }

    /// Function to mutate the action
    pub fn mutate(
        &mut self,
        brain: &Brain,
        parent_team: usize,
        teams: &Vec<usize>,
        p_act_atom: f64,
        learner_id: i32,
    ) {
        // Get the list of the team in the brain.
        let mut list_teams = brain.teams.to_vec();

        // Mutation action
        if flip(p_act_atom) {
            let mut options: Vec<i32> = Vec::new();
            let actions_codes: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

            let team_mutate = list_teams.get_mut(self.action_team.unwrap()).unwrap();

            for action in actions_codes {
                if action != self.action_code {
                    options.push(action);
                }
            }

            if !self.is_atomic() {
                println!(
                    "Learner {} switching from Team {} to atomic action",
                    learner_id,
                    brain.teams[self.action_team.unwrap()].get_id()
                );
                team_mutate.remove_learner(learner_id.try_into().unwrap());
            }

            let new_action_code = options.choose(&mut rand::thread_rng()).unwrap();
            self.action_code = *new_action_code;
            self.action_team = None;
        }
        // Mutation team
        else {
            let mut selection_pool: Vec<usize> = Vec::new();

            for t in teams {
                if *t != self.action_team.unwrap() && *t != parent_team {
                    selection_pool.push(*t);
                }
            }

            if !selection_pool.is_empty() {
                let mut old_team_id: Option<i32> = None;
                if !self.is_atomic() {
                    old_team_id = Some(brain.teams[self.action_team.unwrap()].get_id());
                }

                let new_action_team = selection_pool.choose(&mut rand::thread_rng()).unwrap();
                self.action_team = Some(*new_action_team);

                let new_team_id: i32 = brain.teams[self.action_team.unwrap()].get_id();
                if let Some(old_id) = old_team_id {
                    println!(
                        "Learner {} switched from Team {} to Team {}",
                        learner_id, old_id, new_team_id
                    );
                }
            }
        }
    }
}
