use lp_modeler::dsl::*;
use lp_modeler::solvers::{CbcSolver, SolverTrait, Status};
use problem::{Hero, Problem};

use std::ops::Add;

// Describes the behaviour of combat throughout the time
// that a hero does not die.
#[derive(Debug, PartialEq, Eq)]
struct CombatPhase {
    end_stage: usize,
    early_damage: i32,
}

struct DynVariable {
    index: usize,
    rep: LpInteger,
    label: String,
}

#[allow(dead_code)]
impl DynVariable {
    pub fn new(name: &str, index: usize) -> DynVariable {
        let label = format!("{}_{}", name, index);
        DynVariable {
            index,
            rep: LpInteger::new(label.as_str()),
            label,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Combat {
    phases: Vec<CombatPhase>,
    late_damage: i32,
    chosen_damage: i32,
}

#[allow(dead_code)]
pub fn solve(problem: &Problem) -> Option<Vec<usize>> {
    let combat = build_combat(problem);
    let boosts: Vec<DynVariable> = (0..=problem.chosen_hero)
        .map(|i| DynVariable::new("boost", i))
        .collect();
    let chosen_hero_boost = &boosts.last().unwrap();
    let mut stages: Vec<LpBinary> = Vec::new();
    let mut stage_labels: Vec<String> = Vec::new();
    let mut lp = LpProblem::new("Monster", LpObjective::Maximize);

    {
        let max_boost_damage = problem.max_boosts as i32 * problem.boost_damage;
        let max_total_damage = 10000; // TODO: compute

        let mut remaining_health = problem.monster_health;
        let mut active_phase = 0;

        set_max_boosts(&mut lp, &boosts, problem.max_boosts);

        for turn in 0..combat.phases.last().unwrap().end_stage {
            if turn == combat.phases[active_phase].end_stage {
                active_phase += 1;
            }
            remaining_health -= combat.phases[active_phase].early_damage;

            if (turn as i32 + 1) * max_boost_damage + combat.chosen_damage >= remaining_health {
                stage_labels.push(format!("stage_{}", turn));
                stages.push(LpBinary::new(stage_labels.last().unwrap().as_str()));
                let stage = &stages.last().unwrap();

                let mut post_chosen_constraints =
                    get_boost_contributions(&combat, &boosts, problem.boost_damage, turn);
                let mut pre_chosen_constraints = post_chosen_constraints.clone();
                // Remove the contribution of the chosen hero for this turn and add what it
                // was on the previous turn.
                pre_chosen_constraints.pop();
                pre_chosen_constraints
                    .push(turn as i32 * problem.boost_damage * &chosen_hero_boost.rep);

                // If the stage has been chosen, we need an upper bound on the damage done
                // before the chosen hero and a lower bound on the damage done by the chosen
                // hero themselves.
                // We add large constants to ensure that when the stage is not selected,
                // these constraints have no effect.
                pre_chosen_constraints.push(max_total_damage * *stage);
                post_chosen_constraints.push(-max_total_damage * *stage);

                // -1 to be exclusive, we don't want the monster to die yet.
                lp += lp_sum(&pre_chosen_constraints).le(remaining_health + max_total_damage - 1);
                lp += lp_sum(&post_chosen_constraints)
                    .ge(remaining_health - combat.chosen_damage - max_total_damage);
            }

            remaining_health -= combat.chosen_damage + combat.late_damage;

            if remaining_health <= 0 {
                break;
            }
        }

        // At no stage can the monster be slain.
        if stages.is_empty() {
            return None;
        }

        lp += lp_sum(&stages);
    }

    let solver = CbcSolver::new();
    let (status, results) = solver.run(&lp).unwrap();
    match status {
        Status::Infeasible => None,
        Status::Optimal => {
            if stage_labels.iter().all(|label| results[label] == 0.0) {
                return None;
            }
            let mut result = Vec::new();
            for boost in &boosts {
                for _ in 0..(results[&boost.label] as usize) {
                    result.push(boost.index)
                }
            }
            Some(result)
        }
        _ => {
            panic!(format!("Unexpected status: {:?}", status));
        }
    }
}

fn build_combat(problem: &Problem) -> Combat {
    let early_heroes = &problem.heroes[0..problem.chosen_hero];
    let chosen_hero = &problem.heroes[problem.chosen_hero];
    let late_heroes = &problem.heroes[problem.chosen_hero + 1..problem.heroes.len()];

    let mut phases = Vec::with_capacity(problem.chosen_hero + 1);
    let cumulative_damage = build_cumulative_damage(early_heroes);
    let cumulative_health = cumulative_sum(early_heroes.iter().map(|x| x.health));

    for i in 0..early_heroes.len() {
        phases.push(CombatPhase {
            end_stage: cumulative_health[i],
            early_damage: cumulative_damage[i],
        });
    }
    phases.push(CombatPhase {
        end_stage: cumulative_health.last().unwrap_or(&0) + chosen_hero.health,
        early_damage: 0,
    });

    Combat {
        phases,
        late_damage: late_heroes.iter().map(|x| x.damage).sum(),
        chosen_damage: problem.heroes[problem.chosen_hero].damage,
    }
}

fn cumulative_sum<T: Add<Output = T> + Default + Copy, I>(xs: I) -> Vec<T>
where
    I: Iterator<Item = T>,
{
    let mut total = T::default();
    let mut sums = Vec::new();
    for x in xs {
        total = total.add(x);
        sums.push(total);
    }
    sums
}

fn build_cumulative_damage(heroes: &[Hero]) -> Vec<i32> {
    let mut cumulative_damage = cumulative_sum(heroes.iter().map(|x| x.damage).rev());
    cumulative_damage.reverse();
    cumulative_damage
}

fn get_boost_contributions(
    combat: &Combat,
    boosts: &[DynVariable],
    boost_damage: i32,
    turn: usize,
) -> Vec<LpExpression> {
    combat
        .phases
        .iter()
        .zip(boosts.iter())
        .map(|(phase, boost)| {
            let damage = if phase.end_stage <= turn {
                phase.end_stage as i32 * boost_damage
            } else {
                (turn as i32 + 1) * boost_damage
            };
            damage * &boost.rep
        })
        .collect()
}

#[allow(clippy::identity_op)]
fn set_max_boosts(lp: &mut LpProblem, boosts: &[DynVariable], max_boosts: usize) {
    // Yes, the 1* looks weird, but I can't find a different way to convert an LpInteger
    // to an LpExpression without borrowing it.
    let boost_exprs: Vec<LpExpression> = boosts.iter().map(|boost| 1 * &boost.rep).collect();
    *lp += lp_sum(&boost_exprs).le(max_boosts as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_simple_combat() {
        assert_eq!(
            Combat {
                phases: vec![
                    CombatPhase {
                        end_stage: 10,
                        early_damage: 30
                    },
                    CombatPhase {
                        end_stage: 25,
                        early_damage: 20
                    },
                    CombatPhase {
                        end_stage: 45,
                        early_damage: 0
                    }
                ],
                chosen_damage: 50,
                late_damage: 30,
            },
            build_combat(&Problem {
                monster_health: 100,
                heroes: vec![
                    Hero {
                        health: 10,
                        damage: 10,
                    },
                    Hero {
                        health: 15,
                        damage: 20,
                    },
                    Hero {
                        health: 20,
                        damage: 50,
                    },
                    Hero {
                        health: 10,
                        damage: 30,
                    },
                ],
                chosen_hero: 2,
                boost_damage: 0,
                max_boosts: 0,
            })
        );
    }

    #[test]
    fn various_build_cumulative_damage() {
        assert!(build_cumulative_damage(&[]).is_empty());
        assert_eq!(
            vec![10],
            build_cumulative_damage(&[Hero {
                health: 10,
                damage: 10
            }])
        );
        assert_eq!(
            vec![10, 3],
            build_cumulative_damage(&[
                Hero {
                    health: 10,
                    damage: 7
                },
                Hero {
                    health: 10,
                    damage: 3
                }
            ])
        );
        assert_eq!(
            vec![15, 8, 5],
            build_cumulative_damage(&[
                Hero {
                    health: 10,
                    damage: 7
                },
                Hero {
                    health: 10,
                    damage: 3
                },
                Hero {
                    health: 10,
                    damage: 5
                }
            ])
        );
    }

    #[test]
    fn various_cumulative_sum() {
        assert_eq!(vec![10], cumulative_sum([10].iter().map(|&x| x)));
        assert_eq!(vec![5, 8], cumulative_sum([5, 3].iter().map(|&x| x)));
        assert_eq!(
            vec![5, 12, 15],
            cumulative_sum([5, 7, 3].iter().map(|&x| x))
        );
    }
}
