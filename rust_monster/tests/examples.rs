extern crate rust_monster;
use rust_monster::problem::{Hero, Problem};
use rust_monster::simulation::solution_is_valid;
use rust_monster::solution::solve;

fn assert_solveable(problem: &Problem) {
    match solve(problem) {
        Some(solution) => {
            if !solution_is_valid(problem, &solution) {
                panic!("Invalid solution: {:?}", solution);
            }
        }
        None => panic!("Expected to find a solution."),
    }
}

fn assert_not_solveable(problem: &Problem) {
    match solve(problem) {
        Some(solution) => {
            panic!("Expected unsolvable problem, got solution {:?}", solution);
        }
        None => {}
    }
}

#[test]
fn war_of_attrition() {
    assert_solveable(&Problem {
        monster_health: 100,
        heroes: vec![Hero {
            health: 20,
            damage: 1,
        }],
        chosen_hero: 0,
        boost_damage: 6,
        max_boosts: 1,
    });
}

#[test]
fn barely_enough() {
    assert_solveable(&Problem {
        monster_health: 10,
        heroes: vec![Hero {
            health: 2,
            damage: 4,
        }],
        chosen_hero: 0,
        boost_damage: 1,
        max_boosts: 1,
    });
}

#[test]
fn looser_bounds() {
    assert_solveable(&Problem {
        monster_health: 10,
        heroes: vec![Hero {
            health: 3,
            damage: 4,
        }],
        chosen_hero: 0,
        boost_damage: 1,
        max_boosts: 1,
    });
}

#[test]
fn the_dynamic_duo() {
    assert_solveable(&Problem {
        monster_health: 200,
        heroes: vec![
            Hero {
                health: 2,
                damage: 5,
            },
            Hero {
                health: 1,
                damage: 20,
            },
        ],
        chosen_hero: 1,
        boost_damage: 10,
        max_boosts: 5,
    });
}

#[test]
fn woefully_underleveled() {
    assert_not_solveable(&Problem {
        monster_health: 100,
        heroes: vec![
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 2,
            },
            Hero {
                health: 1,
                damage: 3,
            },
        ],
        chosen_hero: 0,
        boost_damage: 5,
        max_boosts: 5,
    });
}

#[test]
fn unfortunate_overkill() {
    assert_not_solveable(&Problem {
        monster_health: 200,
        heroes: vec![
            Hero {
                health: 2,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 5,
            },
            Hero {
                health: 2,
                damage: 100,
            },
            Hero {
                health: 1,
                damage: 1,
            },
        ],
        chosen_hero: 3,
        boost_damage: 5,
        max_boosts: 3,
    });
}

#[test]
fn strength_in_numbers() {
    assert_solveable(&Problem {
        monster_health: 25,
        heroes: vec![
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 1,
            },
            Hero {
                health: 1,
                damage: 1,
            },
        ],
        chosen_hero: 7,
        boost_damage: 1,
        max_boosts: 2,
    });
}
