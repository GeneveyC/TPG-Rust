//! # Team
//! Crate that permit to represent the team of the Tangled Program Graph (TPG).

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use rand::seq::SliceRandom;

use crate::{flip, Brain, Learner, TeamParams};
use core::panic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
/// Structure to represent the team.
pub struct Team {
    /// The id of the team.
    id: i32,
    /// The list of learner of the team.
    learners: Vec<usize>,
    /// The outcome of the team (one per task).
    outcomes: HashMap<String, i32>,
    /// The fitness of the team.
    fitness: i32,
    /// The learner associated to the team
    in_learners: Vec<usize>,
}

impl PartialEq for Team {
    /// Function to compare two team between them.
    fn eq(&self, other: &Self) -> bool {
        self.learners == other.learners
    }
}

impl Team {
    /// Constructor of the team.
    pub fn new(init_params: &mut TeamParams) -> Self {
        let id: i32 = init_params.get_new_id_team();
        let learners: Vec<usize> = Vec::new();
        let outcomes: HashMap<String, i32> = HashMap::new();
        let fitness: i32 = 0;
        let in_learners: Vec<usize> = Vec::new();
        Team {
            id,
            learners,
            outcomes,
            fitness,
            in_learners,
        }
    }

    /// Function to get the learner of team
    pub fn get_learners(&self) -> Vec<usize> {
        self.learners.to_vec()
    }

    /// Function to get the number of learner in the team
    pub fn get_len_in_learners(&self) -> usize {
        self.in_learners.len()
    }

    /// Function to get the fitness of teams
    pub fn get_fitness(&self) -> i32 {
        self.fitness
    }

    /// Function to set the fitness to teams
    pub fn set_fitness(&mut self, fitness: i32) {
        self.fitness = fitness;
    }

    /// Function to return if the task is in the team
    pub fn is_task_in_outcome(&self, task: String) -> bool {
        self.outcomes.contains_key(&task)
    }

    /// Function to get the score of specific task
    pub fn get_score_of_task(&self, task: String) -> i32 {
        let mut score = 0;
        if let Some(score_for_task) = self.outcomes.get(&task) {
            score = *score_for_task;
        }
        score
    }

    /// Function to return the id of the team.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Function to remove all learner from the team.
    pub fn remove_learners(&mut self) {
        self.learners.clear();
    }

    /// Function to remove the learner with the current idx.
    pub fn remove_learner(&mut self, index: usize) {
        self.in_learners.remove(index);
    }

    /// Function to append a new learner in the list.
    pub fn add_learner(&mut self, learner_idx: usize) {
        self.learners.push(learner_idx);
    }

    /// Function to set the outcomes.
    pub fn set_outcomes(&mut self, task: String, outcome: i32) {
        self.outcomes.remove(&task);
        self.outcomes.insert(task, outcome);
    }

    /// Function to check if the task is done.
    pub fn task_done(&self, task: String) -> bool {
        for key in self.outcomes.keys() {
            if *key == task {
                return true;
            }
        }
        false
    }

    /// Function to reset the register of the learner
    pub fn zero_registers(&mut self, brain: &Brain) {
        // Get the list of learner in the brain
        let mut list_learner = brain.learners.to_vec();
        for idx_learner in &self.learners {
            let learner_mut = list_learner.get_mut(*idx_learner).unwrap();
            learner_mut.zero_registers();
        }
    }

    /// Function to get the number of learner referencing in the team
    pub fn num_learners_referencing(&self) -> i32 {
        self.in_learners.len().try_into().unwrap()
    }

    /// Function to get the number of atomic action in the team
    pub fn num_atomic_actions(&self, brain: &Brain) -> i32 {
        let mut num_atomic_actions: i32 = 0;
        for lrnr_idx in self.learners.iter() {
            let learner = brain.learners.get(*lrnr_idx).unwrap();
            if learner.is_action_atomic(brain) {
                num_atomic_actions += 1;
            }
        }
        num_atomic_actions
    }

    /// Function to act
    pub fn act(&self, brain: &Brain, state: &Vec<i32>, visited: &mut Vec<i32>) -> i32 {
        if visited.contains(&self.id) {
            panic!("Already visited team {}!", self.id);
        }

        // Put the current team to the list of already visited team.
        visited.push(self.id);

        // A Learner is valid if he has atomic action or the team reference with the action is not in the list of team already visited !
        let mut valid_learners: Vec<usize> = Vec::new();
        for lrnr in &self.learners {
            let mut team_already_visited: bool = false;
            if let Some(team_tmp) = brain.learners[*lrnr].get_action_team(brain) {
                let team_id = brain.teams[team_tmp].get_id();
                if visited.contains(&team_id) {
                    team_already_visited = true;
                }
            }

            let learner_with_atomic_action = brain.learners[*lrnr].is_action_atomic(brain);
            if learner_with_atomic_action || !team_already_visited {
                valid_learners.push(*lrnr);
            }
        }

        // Get the list of learner from the brain
        let mut list_learner = brain.learners.to_vec();

        println!("Number of learners in the team: {}", self.learners.len());

        // Compute top learner (with highest bid)
        let learner_mut = list_learner.get_mut(self.learners[0]).unwrap();
        let mut max_bid = learner_mut.bid(state);
        let mut idx_top_learner = self.learners[0];
        println!("Learner n°{} => Bid: {}", idx_top_learner, max_bid);

        for i in 1..self.learners.len() {
            let lrnr = self.learners[i];
            let learner_mut = list_learner.get_mut(lrnr).unwrap();
            let bid = learner_mut.bid(state);
            println!("Learner n°{} => Bid: {}", lrnr, bid);

            if bid > max_bid {
                max_bid = bid;
                idx_top_learner = lrnr;
            }
        }

        println!("The best learner is {}", idx_top_learner);

        // Make path_trace
        brain.learners[idx_top_learner].get_action(brain, state, visited)
    }

    /// Executes a delete mutation with a certain probability
    ///     - Returns immediately if the probability of deletion is 0.0
    ///     - Raise a panic if the probability to deletion is 1.0 or greater as than would simply remove most learners from the teams.
    ///     - Will not delte any learners if there are 2 or fewer learners on the team.
    ///     - Verifies that there is always at least one learner pointing to an atomic action on a team, raise an panic otherwise.
    ///     - If there is only one learner pointing to an atomic filter it out and pick from the remaining learners.
    ///     - Return a list of learners removed from the team
    pub fn mutation_delete(&mut self, brain: &Brain, probability: f64) -> Vec<usize> {
        let mut deleted_learner: Vec<usize> = Vec::new();
        if probability == 0.0 {
            return deleted_learner;
        }

        if probability >= 1.0 {
            panic!("p_lrn_del is greater than or equal to 1.0!");
        }

        if self.num_atomic_actions(brain) < 1 {
            panic!("Less than one atomic action in team! This shouldn't happen");
        }

        while flip(probability) && self.learners.len() > 2 {
            let list_learner = self.learners.to_vec();

            let mut idx_learner: usize = *list_learner.choose(&mut rand::thread_rng()).unwrap();
            if self.num_atomic_actions(brain) < 1 {
                let mut valid_learner: Vec<usize> = Vec::new();
                for idx_lrnr in list_learner {
                    let learner = brain.learners.get(idx_lrnr).unwrap();
                    if !learner.is_action_atomic(brain) {
                        valid_learner.push(idx_lrnr);
                    }
                }
                idx_learner = *valid_learner.choose(&mut rand::thread_rng()).unwrap();
            }
            deleted_learner.push(idx_learner);
            self.remove_learner(idx_learner);
        }

        deleted_learner
    }

    /// A learner is added from the provided selection pool with a given probability.
    pub fn mutation_add(
        &mut self,
        probability: f64,
        max_team_size: usize,
        selection_pool: &mut Vec<usize>,
    ) -> Vec<usize> {
        let mut added_learner: Vec<usize> = Vec::new();

        if probability == 0.0
            || selection_pool.is_empty()
            || (max_team_size > 0 && self.learners.len() >= max_team_size)
        {
            return added_learner;
        }

        if probability >= 1.0 {
            panic!("p_lrn_add is greather than or equal to 1.0!");
        }

        while flip(probability) && (max_team_size == 0 || self.learners.len() < max_team_size) {
            if selection_pool.is_empty() {
                break;
            }

            let idx_learner = selection_pool.choose(&mut rand::thread_rng()).unwrap();
            added_learner.push(*idx_learner);

            self.add_learner(*idx_learner);

            let selection_pool_tmp = selection_pool.to_vec();
            selection_pool.clear();
            for select_tmp in selection_pool_tmp {
                if !added_learner.contains(&select_tmp) {
                    selection_pool.push(select_tmp);
                }
            }
        }
        added_learner
    }

    /// Iterate throught this team's learners and mutates them with a given probability.
    pub fn mutation_mutate(
        &mut self,
        brain: &mut Brain,
        probability: f64,
        mutate_params: &mut TeamParams,
        teams: &Vec<usize>,
    ) -> (HashMap<i32, i32>, Vec<usize>) {
        // Create a list to store the mutation of current learner to other learner
        let mut mutate_learner: HashMap<i32, i32> = HashMap::new();

        // Save the list of original learner
        let original_learner = self.learners.to_vec();

        // Create a list of new learner
        let mut new_learners: Vec<usize> = Vec::new();

        // For the existing learner
        for idx_lrnr in original_learner {
            // Get the learner struct
            let learner = brain.learners.get(idx_lrnr).unwrap();
            // Get the learner id
            let learner_id = learner.get_id();

            // If we apply the mutation on the current learner
            if flip(probability) {
                let mut p_act_atom0: f64 = mutate_params.p_act_atom;
                if self.num_atomic_actions(brain) == 1 && learner.is_action_atomic(brain) {
                    p_act_atom0 = 1.1;
                }

                let mut new_learner = Learner::new(
                    &mut mutate_params.learner,
                    learner.get_program(),
                    learner.get_idx_action(),
                    learner.get_len_register(),
                );

                let parent_team = brain.get_team_index_from_team_id(self.get_id()).unwrap();

                // Mutate it
                new_learner.mutate(
                    brain,
                    &mutate_params.learner,
                    parent_team,
                    teams,
                    p_act_atom0,
                );

                let id_new_learner = new_learner.get_id();

                new_learners.push(brain.learners.len());

                brain.add_learner(new_learner);

                // Add the mutate learner to our list of mutations
                mutate_learner.insert(learner_id, id_new_learner);

                // Remove the existing learner from the team.
                self.remove_learner(idx_lrnr);
            }
        }
        (mutate_learner, new_learners)
    }

    /// Mutates the learner set of this team.
    pub fn mutate(
        &mut self,
        brain: &mut Brain,
        mutate_params: &mut TeamParams,
        all_learners: Vec<usize>,
        teams: Vec<usize>,
    ) -> i32 {
        if mutate_params.rampant_gen != 0 && mutate_params.rampant_min > mutate_params.rampant_max {
            panic!("Min rampant iterations is greather than max rampant iterations!");
        }

        let rampant_rep = 1;

        let mut new_learner: Vec<usize> = Vec::new();

        for i in 0..rampant_rep {
            println!("i/rampat reps: {}/{}", i, rampant_rep);

            let deleted_learners = self.mutation_delete(brain, mutate_params.p_lrn_del);

            // Filter out learners that already belong to this team
            let mut selection_pool1: Vec<usize> = Vec::new();
            for learner_idx in &all_learners {
                if !self.learners.contains(learner_idx) {
                    selection_pool1.push(*learner_idx);
                }
            }

            // Filter out learners that point to this team
            let mut selection_pool2: Vec<usize> = Vec::new();
            for learner_idx in &selection_pool1 {
                if self.in_learners.contains(learner_idx) {
                    selection_pool2.push(*learner_idx);
                }
            }

            // Filter out learners we just deleted
            let mut selection_pool3: Vec<usize> = Vec::new();
            for learner_idx in &selection_pool2 {
                if !deleted_learners.contains(learner_idx) {
                    selection_pool3.push(*learner_idx);
                }
            }

            let _added_learner =
                self.mutation_add(mutate_params.p_lrn_add, 10, &mut selection_pool3);

            // give a chance to mutate all learners
            let results_mutation =
                self.mutation_mutate(brain, mutate_params.p_lrn_mut, mutate_params, &teams);

            let mutation_added_learners = results_mutation.1;

            for learner_idx in &mutation_added_learners {
                new_learner.push(*learner_idx);
            }
        }

        let new_learner_tmp = new_learner.clone();
        for learner_idx in &new_learner_tmp {
            if self.learners.contains(learner_idx) {
                new_learner.remove(*learner_idx);
            }
        }

        for learner_idx in &new_learner {
            let learner = brain.learners.get(*learner_idx).unwrap();
            if learner.num_teams_referencing() == 0 && !learner.is_action_atomic(brain) {
                if let Some(team_idx) = learner.get_action_team(brain) {
                    let team = brain.teams.get_mut(team_idx).unwrap();
                    team.remove_learner(*learner_idx);
                }
            }
        }

        rampant_rep
    }
}
