//! # Trainer
//! Crate that permit to represent the trainer of the Tangled Program Graph (TPG).

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::missing_docs_in_private_items)]

use crate::{TrainerParams, Action, Agent, Brain, Learner, Program, Team};
use rand::{seq::SliceRandom, Rng};

/// Structure to represent the trainer
#[derive(Default)]
pub struct Trainer {
    /// Bool to say if we concerve elites
    do_elites: bool,
    /// The number of register
    n_register: usize,
    /// The list of teams
    teams: Vec<usize>,
    /// The list of root teams
    root_teams: Vec<usize>,
    /// The list of learner
    learners: Vec<usize>,
    /// The list of elite team
    elites: Vec<usize>,
    /// The generation
    generation: i32,
    /// Initial number of teams / root teams to be maintained through evolution.
    team_pop_size: i32,
    /// List of the action.
    action_codes: Vec<i32>,
    /// The number of action
    action_lengths: usize,
    /// The trainer information
    mutate_params: TrainerParams,
}

impl Trainer {
    /// Constructor of the trainer
    pub fn new(init_params: &mut TrainerParams, n_register: usize) -> Self {
        let do_elites = true;
        let teams: Vec<usize> = Vec::new();
        let root_teams: Vec<usize> = Vec::new();
        let learners: Vec<usize> = Vec::new();
        let elites: Vec<usize> = Vec::new();
        let generation: i32 = 0;

        let team_pop_size = 5;
        let action_codes: Vec<i32> = Vec::new();
        let action_lengths: usize = 0;
        let mutate_params: TrainerParams = init_params.clone();

        Trainer {
            do_elites,
            n_register,
            teams,
            root_teams,
            learners,
            elites,
            generation,
            team_pop_size,
            action_codes,
            action_lengths,
            mutate_params,
        }
    }

    /// Function to get the number of teams in the trainer
    pub fn get_nb_teams(&self) -> usize {
        self.teams.len()
    }

    /// Function to setup actions
    pub fn set_up_actions(&mut self, actions: Vec<i32>) {
        if actions.is_empty() {
            panic!("The actions is empty !");
        } else {
            // for i in 0..actions.len() {
            for action in &actions {
                self.action_codes.push(*action);
            }
            self.action_lengths = actions.len();
        }
    }

    /// The initialization of populations
    pub fn initialize_populations(&mut self, brain: &mut Brain) {
        // Check that the list of action is not empty.
        if self.action_codes.len() < 2 {
            panic!("The list of action codes is empty !");
        }

        // For each team in the population.
        for _i in 0..self.team_pop_size {
            // Choose 2 unique actions in the list of action.
            let a1 = self.action_codes.choose(&mut rand::thread_rng()).unwrap();
            let mut a2 = self.action_codes.choose(&mut rand::thread_rng()).unwrap();
            while *a2 == *a1 {
                a2 = self.action_codes.choose(&mut rand::thread_rng()).unwrap();
            }

            // Create the first action.
            let action1 = Action::new(
                *a1,
                &mut self
                    .mutate_params
                    .team
                    .learner
                    .action,
            );

            let action1_idx = brain.actions.len();

            // Add the first action in the brain.
            brain.add_action(action1);

            // Create the second action in the brain.
            let action2 = Action::new(
                *a2,
                &mut self
                    .mutate_params
                    .team
                    .learner
                    .action,
            );
            let action2_idx = brain.actions.len();

            // Add the second action in the brain
            brain.add_action(action2);

            // Create the two program for the learner 1 and learner 2.
            let program1: Program = Program::new(
                None,
                &mut self
                    .mutate_params
                    .team
                    .learner
                    .program,
            );

            let program2: Program = Program::new(
                None,
                &mut self
                    .mutate_params
                    .team
                    .learner
                    .program,
            );

            // Create two learner based on the two action and append it in the brain.
            let learner1: Learner = Learner::new(
                &mut self.mutate_params.team.learner,
                program1.clone(),
                action1_idx,
                self.n_register,
            );
            let learner1_idx = brain.learners.len();

            // Add the learner 1 in the brain.
            brain.add_learner(learner1.clone());

            let learner2: Learner = Learner::new(
                &mut self.mutate_params.team.learner,
                program2.clone(),
                action2_idx,
                self.n_register,
            );
            let learner2_idx = brain.learners.len();

            // Add the learner 2 in the brain.
            brain.add_learner(learner2.clone());

            // Create a new team with the previous learner.
            let mut team: Team = Team::new(&mut self.mutate_params.team);
            team.add_learner(learner1_idx);
            team.add_learner(learner2_idx);

            if team.get_learners().len() != 2 {
                panic!("They don't have two learner in the team!");
            }

            // Add more learners
            let mut rng = rand::thread_rng();
            let more_learners = rng.gen_range(0..self.mutate_params.max_learner_in_team - 2);

            // If me add more learners
            for _i in 0..more_learners {
                // Select action
                let act = self.action_codes.choose(&mut rand::thread_rng()).unwrap();

                // Create a new action
                let action_tmp = Action::new(
                    *act,
                    &mut self
                        .mutate_params
                        .team
                        .learner
                        .action,
                );
                let action_idx = brain.actions.len();

                // Add action in the brain
                brain.add_action(action_tmp.clone());

                // Create a new program
                let program_tmp = Program::new(
                    None,
                    &mut self
                        .mutate_params
                        .team
                        .learner
                        .program,
                );

                // Create a new learner
                let learner_tmp = Learner::new(
                    &mut self.mutate_params.team.learner,
                    program_tmp,
                    action_idx,
                    self.n_register,
                );
                let learner_idx = brain.learners.len();

                // Add the learner in the brain
                brain.add_learner(learner_tmp.clone());

                // Add the learner in the teams
                team.add_learner(learner_idx);
            }

            let team_idx = brain.teams.len();

            // Save the team in the brain
            brain.add_team(team.clone());

            // Add reference of the team to the trainer
            self.teams.push(team_idx);

            // Add reference of the root team to the trainer
            self.root_teams.push(team_idx);
        }
    }

    /// Function to get the root teams / agents.
    /// Sort decending by sortTasks, and skips individuals who don't have scores for all skip tasks.
    pub fn get_agents(
        &self,
        brain: &Brain,
        sort_tasks: Vec<String>,
        skip_tasks: Vec<String>,
    ) -> Vec<Agent> {
        let mut r_teams: Vec<usize> = Vec::new();
        for team_idx in &self.root_teams {
            let team = brain.teams.get(*team_idx).unwrap();
            let mut task_in_outcomes: bool = false;
            for task in &skip_tasks {
                if team.is_task_in_outcome(task.to_string()) {
                    task_in_outcomes = true;
                }
            }

            if !task_in_outcomes {
                r_teams.push(*team_idx);
            }
        }

        let mut list_agents: Vec<Agent> = Vec::new();
        if sort_tasks.is_empty() {
            for team_idx in r_teams.iter() {
                let agent: Agent = Agent::new(*team_idx);
                list_agents.push(agent);
            }
            list_agents
        } else if sort_tasks.len() == 1 {
            let mut r_teams2: Vec<usize> = Vec::new();
            for team_idx in &r_teams {
                let team = brain.teams.get(*team_idx).unwrap();
                if team.is_task_in_outcome(sort_tasks[0].to_string()) {
                    r_teams2.push(*team_idx);
                }
            }

            /*
                TODO (Maybe sort struct by specific fitness ? )
                For the moment just return the better agent
            */
            let mut best_team_idx: usize = 0;
            let mut best_fitness: i32 = 0;
            for team_idx in r_teams.iter() {
                let team = brain.teams.get(*team_idx).unwrap();
                let fitness = team.get_score_of_task(sort_tasks[0].to_string());
                if fitness > best_fitness {
                    best_fitness = fitness;
                    best_team_idx = *team_idx;
                }
            }
            
            let agent: Agent = Agent::new(best_team_idx);
            list_agents.push(agent);
            list_agents
        } else {
            panic!("Multi-task not implemented for the moment!");
        }
    }

    /// Function to get the elite agents of trainer
    pub fn get_elite_agent(&self, brain: &Brain, task: String) -> Agent {
        let mut teams: Vec<usize> = Vec::new();
        for t_idx in &self.teams {
            let team = brain.teams.get(*t_idx).unwrap();
            if team.is_task_in_outcome(task.to_string()) {
                teams.push(*t_idx);
            }
        }

        let mut best_team_idx: usize = 0;
        let mut best_score: i32 = 0;

        for team_idx in teams {
            let team = brain.teams.get(team_idx).unwrap();
            let score = team.get_score_of_task(task.to_string());
            if score > best_score {
                best_score = score;
                best_team_idx = team_idx;
            }
        }

        Agent::new(best_team_idx)
    }

    /// Function to evolve the trainer
    pub fn evolve(&mut self, brain: &mut Brain, tasks: Vec<String>, extra_teams: &Vec<usize>) {
        // Assign score to individuals
        println!("Before the score individuals !");
        self.score_individuals(brain, tasks, self.do_elites);
        // Select individuals to keep based on their fitness
        println!("Before the seclection of the inviduals !");
        self.select(brain, extra_teams);
        // Create a new individuals from those kept
        println!("Before the generation of new populations !");
        self.generate(brain, extra_teams);
        // Set up for the next generation
        println!("Before the next epoch !");
        self.next_epoch(brain);
    }

    /// Assign a fitness to each agent based on performance at the tasks.
    /// Assigns fitness value, or just returns sorted root teams.
    pub fn score_individuals(&mut self, brain: &mut Brain, tasks: Vec<String>, do_elites: bool) {
        // handle generation of new elites, typically just done in evolution

        if do_elites {
            // save the best root team for each task based on their score for a task.
            self.elites.clear();

            for task in &tasks {
                let mut best_team: usize = 0;
                let mut best_score: i32 = 0;
                for team_idx in &self.root_teams {
                    let team = brain.teams.get(*team_idx).unwrap();
                    let score = team.get_score_of_task(task.to_string());
                    if score > best_score {
                        best_score = score;
                        best_team = *team_idx;
                    }
                }
                self.elites.push(best_team);
            }
        }

        // if single task
        if tasks.len() == 1 {
            for team_idx in self.root_teams.iter() {
                let team = brain.teams.get_mut(*team_idx).unwrap();
                team.set_fitness(team.get_score_of_task(tasks[0].to_string()));
            }
        } else {
            panic!("The multi-objective is not define for the moment");
        }
    }

    /// Select a portion of the root team population to keep according to gap size.
    pub fn select(&mut self, brain: &mut Brain, extra_teams: &[usize]) {
        // Get the new list of team idx sorted by fitness.
        let ranked_team_idx: Vec<usize> = brain.sort_teams_idx_with_fitness(&self.root_teams);
        println!(
            "The list of teams (ordered by fitness) = {:#?}",
            ranked_team_idx
        );

        let crossover = 0.5;
        let num_keep_float =
            ranked_team_idx.len() as f64 - (ranked_team_idx.len() as f64 * crossover);
        let num_keep: usize = num_keep_float as usize;

        // add the idx of deleted teams.
        let mut deleted_teams: Vec<usize> = Vec::new();
        for team_idx in ranked_team_idx.iter().skip(num_keep) {
            deleted_teams.push(*team_idx);
        }

        // For all learner that don't have a number of reference teams, we delete it (manage memory)
        // CHECK: If this case is not always false ? A learner need to always have a reference teams.
        let mut pre_orphans: Vec<usize> = Vec::new();
        for learner_idx in &self.learners {
            let learner = brain.learners.get(*learner_idx).unwrap();
            if learner.num_teams_referencing() == 0 {
                pre_orphans.push(*learner_idx);
            }
        }

        // For all teams that don't have learners and not contains in the root teams, we delete it (manage memory).
        // CHECK: If this case is not always false ? A Team need to always have a learner.
        let mut orphan_teams: Vec<usize> = Vec::new();
        for team_idx in &self.teams {
            let team = brain.teams.get(*team_idx).unwrap();
            if team.get_len_in_learners() == 0 && self.root_teams.contains(team_idx) {
                orphan_teams.push(*team_idx);
            }
        }

        // We keep to the delete teams all team that are not in the elite team.
        let mut team_select: Vec<usize> = Vec::new();
        for team_idx in deleted_teams {
            if !self.elites.contains(&team_idx) {
                team_select.push(team_idx);
            }
        }

        for team_idx in team_select {
            // remove learners from teams and delete team from population
            if !extra_teams.contains(&team_idx) {
                let team = brain.teams.get_mut(team_idx).unwrap();
                team.remove_learners();
            }

            // Get the index of the teams_idx in the teams list
            let idx_element_teams = self.teams.iter().position(|x| *x == team_idx).unwrap();
            // Get the index of the teams_idx in the root teams list
            let idx_element_root_teams =
                self.root_teams.iter().position(|x| *x == team_idx).unwrap();

            self.teams.remove(idx_element_teams);
            self.root_teams.remove(idx_element_root_teams);

            // TODO: Maybe remove team from the brain

            // TODO: Clean the link between trainer and brain
        }

        // Manage orphans learner
        let mut orphans: Vec<usize> = Vec::new();
        for learner_idx in &self.learners {
            let learner = brain.learners.get(*learner_idx).unwrap();
            if learner.num_teams_referencing() == 0 {
                orphans.push(*learner_idx);
            }
        }

        for learner_idx in orphans {
            let learner = brain.learners.get(learner_idx).unwrap();
            if learner.is_action_atomic(brain) {
                if let Some(team_idx) = learner.get_action_team(brain) {
                    let team = brain.teams.get_mut(team_idx).unwrap();
                    team.remove_learner(learner_idx);
                }
            }
        }

        let mut new_learner: Vec<usize> = Vec::new();
        for learner_idx in &self.learners {
            let learner = brain.learners.get(*learner_idx).unwrap();
            if learner.num_teams_referencing() > 0 {
                new_learner.push(*learner_idx);
            }
        }

        self.learners.clear();
        self.learners = new_learner.clone();
    }

    /// Generate new root teams based on existing teams.
    pub fn generate(&mut self, brain: &mut Brain, extra_teams: &Vec<usize>) {
        let mut protected_extras: Vec<usize> = Vec::new();
        let mut extras_added: i32 = 0;

        for team_idx in extra_teams {
            if !self.teams.contains(team_idx) {
                self.teams.push(*team_idx);
                extras_added += 1;
            } else {
                protected_extras.push(*team_idx);
            }
        }

        let o_learners: Vec<usize> = self.learners.to_vec();
        let o_teams: Vec<usize> = self.teams.to_vec();

        self.mutate_params.generation = self.generation;

        let mut number_teams = self.teams.len() as i32;
        while number_teams < (self.team_pop_size + extras_added) {
            // get parent root team, and child to be based on that
            let parent_idx = self.root_teams.choose(&mut rand::thread_rng()).unwrap();
            let parent = brain.teams.get(*parent_idx).unwrap();

            let mut child: Team = Team::new(&mut self.mutate_params.team);

            // child start just like parent
            for learner_idx in parent.get_learners() {
                child.add_learner(learner_idx);
            }

            // then mutate
            child.mutate(
                brain,
                &mut self.mutate_params.team,
                o_learners.to_vec(),
                o_teams.to_vec(),
            );

            // add the new child into the brain
            let child_idx = brain.teams.len();

            brain.add_team(child);
            self.teams.push(child_idx);

            number_teams = self.teams.len() as i32;
        }

        for team_idx in extra_teams {
            let team = brain.teams.get(*team_idx).unwrap();
            if team.num_learners_referencing() == 0 && !protected_extras.contains(team_idx) {
                self.teams.remove(*team_idx);
            }
        }
    }

    /// Finalize populations and prepare for the next generation / epoch.
    pub fn next_epoch(&mut self, brain: &Brain) {
        self.root_teams.clear();
        for team_idx in &self.teams {
            let team = brain.teams.get(*team_idx).unwrap();
            // add any new learners to the population
            for learner_idx in &team.get_learners() {
                if !self.learners.contains(learner_idx) {
                    self.learners.push(*learner_idx);
                }
            }

            // maybe make root team
            if team.num_learners_referencing() == 0 || self.elites.contains(team_idx) {
                self.root_teams.push(*team_idx);
            }
        }
        self.generation += 1;
    }
}
