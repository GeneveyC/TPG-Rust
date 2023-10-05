//! # Brain
//! Crate that permit to represent the brain of the Tangled Program Graph (TPG).
//! (The brain is a new concept introduce by me)

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use crate::{Action, Learner, Team};
use std::collections::HashMap;

/// Structure to represent the brain.
#[derive(Default)]
pub struct Brain {
    /// The list of id -> index of the team in the TPG.
    pub team_id_to_index: HashMap<i32, usize>,
    /// The list of teams inside the TPG.
    pub teams: Vec<Team>,
    /// The list of id -> index of the learner in the TPG.
    pub learner_id_to_index: HashMap<i32, usize>,
    /// The list of learner inside the TPG.
    pub learners: Vec<Learner>,
    /// The list of id -> index of the action in the TPG.
    pub action_id_to_index: HashMap<i32, usize>,
    /// The list of actions inside the TPG.
    pub actions: Vec<Action>,
}

impl Brain {
    /// Function to add a learner in the brain.
    pub fn add_learner(&mut self, learner: Learner) {
        self.learners.push(learner);
    }

    /// Function to add a team in the brain.
    pub fn add_team(&mut self, team: Team) {
        self.teams.push(team);
    }

    /// Function to add the action in the brain.
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Function to get the index of the team based on the team id
    pub fn get_team_index_from_team_id(&self, team_id: i32) -> Option<usize> {
        for i in 0..self.teams.len() {
            let team = self.teams.get(i).unwrap();
            if team.get_id() == team_id {
                return Some(i);
            }
        }
        None
    }

    /// Function to get the index of the learner based on the learner id
    pub fn get_learner_index_from_learner_id(&self, learner_id: i32) -> Option<usize> {
        for i in 0..self.learners.len() {
            let learner = self.learners.get(i).unwrap();
            if learner.get_id() == learner_id {
                return Some(i);
            }
        }
        None
    }

    /// Function to get the index of the action based on the action id
    pub fn get_action_index_from_action_id(&self, action_id: i32) -> Option<usize> {
        for i in 0..self.actions.len() {
            let action = self.actions.get(i).unwrap();
            if action.get_id() == action_id {
                return Some(i);
            }
        }
        None
    }

    /// Function to sort a list of teams idx based on their fitness
    pub fn sort_teams_idx_with_fitness(&self, list_teams_idx_no_sort: &Vec<usize>) -> Vec<usize> {
        let mut list_teams_unsorted: Vec<Team> = Vec::new();
        let mut list_teams_idx_sorted: Vec<usize> = Vec::new();

        // Get the list of teams (no sorted)
        for team_idx in list_teams_idx_no_sort {
            let team = self.teams.get(*team_idx).unwrap();
            list_teams_unsorted.push(team.clone());
        }

        let mut list_teams_sorted = list_teams_unsorted.clone();
        list_teams_sorted.sort_by_key(|b| std::cmp::Reverse(b.get_fitness()));

        for team in list_teams_sorted {
            let id_team = team.get_id();
            let idx_team = self.get_team_index_from_team_id(id_team).unwrap();
            list_teams_idx_sorted.push(idx_team);
        }
        list_teams_idx_sorted
    }
}
