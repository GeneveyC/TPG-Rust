use tpg::{Brain, Trainer, Params};

fn main() {
    println!("Test for the TPG");

    let actions: Vec<i32> = vec![0, 1, 2, 3, 4];

    let mut brain = Brain::default();

    let mut params: Params = Params::new("params_trainer.json".to_string());

    let mut trainer = Trainer::new(&mut params.trainer, 5);
    trainer.set_up_actions(actions);

    trainer.initialize_populations(&mut brain);

    println!("Initialization of population terminated!");

    println!("List of teams: {:#?}", brain.teams);

    // Test in several iterations
    for i in 0..100 {
        println!("Iteration = {}/{}", i, 10);

        let sort_tasks: Vec<String> = Vec::new();
        let skip_tasks: Vec<String> = Vec::new();

        let mut agents = trainer.get_agents(&brain, sort_tasks, skip_tasks);
        println!("Before get the first agents");
        
        for agent in agents.iter_mut() {

            let state: Vec<i32> = vec![1, 1, 1, 1];

            let act = agent.act(&mut brain, &state);
            println!("The action take by the agent: {}", act);

            // define a reward for the test (just to test the code)
            let mut score = -100;
            if act == 2 {
                score = 100;
            }

            println!("Before reward the agent");

            // backpropage the reward
            agent.reward(&mut brain, score, "t1".to_string());
        }

        let tasks: Vec<String> = vec!["t1".to_string()];
        let extra_teams: Vec<usize> = Vec::new();

        println!("Before evolve the trainer");
        trainer.evolve(&mut brain, tasks, &extra_teams);
    }

    println!("Teams (in the brain): {:#?}", brain.teams);
}
