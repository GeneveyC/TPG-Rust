# TPG Rust

The goal of this project is to implement  the Tangled Program Graph in Rust.
For this first version, I have adapted the principle of TPG in python created by
Ryan-Amaral (https://github.com/Ryan-Amaral/PyTPG). I also introduce the notion
of brain in order to more easily manage the mutation of teams, learner, and program.
With this new notion all teams, learner, program are push in the Brain structure
and only reference are give to the team, learner, program. This permit to better
manage the dependancy recursivity in TPG.

## How to launch the TPG:
```
cargo r --release
```

## Improvment:
- [ ] Use id of team, learner, and program in the brain whereas the index.
- [ ] Implement a solution to manage other value than integer in the input vector.
- [ ] Fix some bug due to the mutation effect.
- [ ] Clean the brain when the team contains no learners.
- [ ] Implement functionality to save the historic of mutation in the TPG.
- [ ] Think about a parallelized version of the TPG.
- [ ] Implemented Multi-task objective for the TPG.

## Other TPG implementations:
+ https://github.com/Ryan-Amaral/PyTPG
