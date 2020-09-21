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

#[test]
fn healthy_heroes() {
    assert_solveable(&Problem {
        monster_health: 465,
        heroes: vec![
            Hero {
                health: 11,
                damage: 15,
            },
            Hero {
                health: 13,
                damage: 10,
            },
            Hero {
                health: 3,
                damage: 19,
            },
            Hero {
                health: 4,
                damage: 24,
            },
            Hero {
                health: 10,
                damage: 14,
            },
            Hero {
                health: 11,
                damage: 30,
            },
            Hero {
                health: 11,
                damage: 23,
            },
            Hero {
                health: 3,
                damage: 23,
            },
            Hero {
                health: 10,
                damage: 4,
            },
            Hero {
                health: 12,
                damage: 32,
            },
            Hero {
                health: 10,
                damage: 12,
            },
            Hero {
                health: 11,
                damage: 20,
            },
            Hero {
                health: 10,
                damage: 9,
            },
            Hero {
                health: 14,
                damage: 8,
            },
            Hero {
                health: 1,
                damage: 12,
            },
            Hero {
                health: 9,
                damage: 25,
            },
            Hero {
                health: 2,
                damage: 30,
            },
            Hero {
                health: 133,
                damage: 1,
            },
        ],
        chosen_hero: 5,
        boost_damage: 3,
        max_boosts: 9,
    });
}

#[test]
fn large_battle() {
    assert_solveable(&Problem {
        monster_health: 1210,
        heroes: vec![
            Hero {
                health: 5,
                damage: 19,
            },
            Hero {
                health: 5,
                damage: 28,
            },
            Hero {
                health: 5,
                damage: 24,
            },
            Hero {
                health: 1,
                damage: 29,
            },
            Hero {
                health: 5,
                damage: 17,
            },
            Hero {
                health: 5,
                damage: 28,
            },
            Hero {
                health: 7,
                damage: 30,
            },
            Hero {
                health: 5,
                damage: 10,
            },
            Hero {
                health: 6,
                damage: 15,
            },
            Hero {
                health: 8,
                damage: 29,
            },
            Hero {
                health: 8,
                damage: 13,
            },
            Hero {
                health: 2,
                damage: 25,
            },
            Hero {
                health: 2,
                damage: 5,
            },
            Hero {
                health: 6,
                damage: 5,
            },
            Hero {
                health: 4,
                damage: 8,
            },
            Hero {
                health: 5,
                damage: 6,
            },
            Hero {
                health: 7,
                damage: 7,
            },
            Hero {
                health: 8,
                damage: 25,
            },
            Hero {
                health: 2,
                damage: 12,
            },
        ],
        chosen_hero: 11,
        boost_damage: 17,
        max_boosts: 8,
    });
}

#[test]
fn current_year() {
    assert_solveable(&Problem {
        monster_health: 2020,
        heroes: vec![
            Hero {
                health: 1,
                damage: 5,
            },
            Hero {
                health: 2,
                damage: 25,
            },
            Hero {
                health: 2,
                damage: 18,
            },
            Hero {
                health: 1,
                damage: 8,
            },
            Hero {
                health: 2,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 18,
            },
            Hero {
                health: 1,
                damage: 30,
            },
            Hero {
                health: 1,
                damage: 5,
            },
            Hero {
                health: 2,
                damage: 21,
            },
            Hero {
                health: 1,
                damage: 13,
            },
            Hero {
                health: 2,
                damage: 7,
            },
            Hero {
                health: 1,
                damage: 26,
            },
            Hero {
                health: 1,
                damage: 11,
            },
            Hero {
                health: 2,
                damage: 26,
            },
            Hero {
                health: 1,
                damage: 18,
            },
            Hero {
                health: 2,
                damage: 17,
            },
            Hero {
                health: 1,
                damage: 22,
            },
            Hero {
                health: 2,
                damage: 21,
            },
        ],
        chosen_hero: 10,
        boost_damage: 12,
        max_boosts: 3,
    });
}

#[test]
fn the_penultimate() {
    assert_solveable(&Problem {
        monster_health: 9089,
        heroes: vec![
            Hero {
                health: 2,
                damage: 27,
            },
            Hero {
                health: 2,
                damage: 28,
            },
            Hero {
                health: 1,
                damage: 14,
            },
            Hero {
                health: 1,
                damage: 20,
            },
            Hero {
                health: 1,
                damage: 15,
            },
            Hero {
                health: 2,
                damage: 29,
            },
            Hero {
                health: 2,
                damage: 23,
            },
            Hero {
                health: 1,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 8,
            },
            Hero {
                health: 1,
                damage: 21,
            },
            Hero {
                health: 1,
                damage: 9,
            },
            Hero {
                health: 2,
                damage: 17,
            },
            Hero {
                health: 1,
                damage: 24,
            },
            Hero {
                health: 1,
                damage: 22,
            },
            Hero {
                health: 2,
                damage: 23,
            },
            Hero {
                health: 1,
                damage: 8,
            },
            Hero {
                health: 1,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 20,
            },
            Hero {
                health: 2,
                damage: 16,
            },
            Hero {
                health: 1,
                damage: 12,
            },
            Hero {
                health: 2,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 16,
            },
            Hero {
                health: 2,
                damage: 7,
            },
            Hero {
                health: 1,
                damage: 22,
            },
            Hero {
                health: 1,
                damage: 27,
            },
            Hero {
                health: 1,
                damage: 18,
            },
            Hero {
                health: 1,
                damage: 18,
            },
            Hero {
                health: 1,
                damage: 12,
            },
            Hero {
                health: 2,
                damage: 16,
            },
            Hero {
                health: 2,
                damage: 26,
            },
            Hero {
                health: 1,
                damage: 15,
            },
        ],
        chosen_hero: 17,
        boost_damage: 20,
        max_boosts: 5,
    });
}

#[test]
fn high_damage() {
    assert_not_solveable(&Problem {
        monster_health: 5630,
        heroes: vec![
            Hero {
                health: 1,
                damage: 10,
            },
            Hero {
                health: 2,
                damage: 23,
            },
            Hero {
                health: 2,
                damage: 21,
            },
            Hero {
                health: 2,
                damage: 29,
            },
            Hero {
                health: 2,
                damage: 27,
            },
            Hero {
                health: 1,
                damage: 26,
            },
            Hero {
                health: 1,
                damage: 13,
            },
            Hero {
                health: 2,
                damage: 13,
            },
            Hero {
                health: 1,
                damage: 26,
            },
            Hero {
                health: 2,
                damage: 28,
            },
            Hero {
                health: 1,
                damage: 21,
            },
            Hero {
                health: 1,
                damage: 17,
            },
            Hero {
                health: 1,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 23,
            },
            Hero {
                health: 1,
                damage: 24,
            },
            Hero {
                health: 2,
                damage: 20,
            },
        ],
        chosen_hero: 2,
        boost_damage: 9,
        max_boosts: 7,
    });
}

#[test]
fn poor_positioning() {
    assert_not_solveable(&Problem {
        monster_health: 1000000,
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
        chosen_hero: 0,
        boost_damage: 1,
        max_boosts: 10,
    });
}

#[test]
fn the_ultimate() {
    assert_solveable(&Problem {
        monster_health: 9115,
        heroes: vec![
            Hero {
                health: 2,
                damage: 27,
            },
            Hero {
                health: 1,
                damage: 28,
            },
            Hero {
                health: 1,
                damage: 14,
            },
            Hero {
                health: 1,
                damage: 20,
            },
            Hero {
                health: 1,
                damage: 15,
            },
            Hero {
                health: 2,
                damage: 29,
            },
            Hero {
                health: 1,
                damage: 23,
            },
            Hero {
                health: 1,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 8,
            },
            Hero {
                health: 1,
                damage: 21,
            },
            Hero {
                health: 1,
                damage: 9,
            },
            Hero {
                health: 1,
                damage: 17,
            },
            Hero {
                health: 1,
                damage: 24,
            },
            Hero {
                health: 1,
                damage: 22,
            },
            Hero {
                health: 1,
                damage: 23,
            },
            Hero {
                health: 1,
                damage: 8,
            },
            Hero {
                health: 1,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 20,
            },
            Hero {
                health: 2,
                damage: 16,
            },
            Hero {
                health: 1,
                damage: 12,
            },
            Hero {
                health: 2,
                damage: 15,
            },
            Hero {
                health: 1,
                damage: 16,
            },
            Hero {
                health: 2,
                damage: 7,
            },
            Hero {
                health: 1,
                damage: 22,
            },
            Hero {
                health: 1,
                damage: 27,
            },
            Hero {
                health: 1,
                damage: 18,
            },
            Hero {
                health: 1,
                damage: 18,
            },
            Hero {
                health: 1,
                damage: 12,
            },
            Hero {
                health: 1,
                damage: 16,
            },
            Hero {
                health: 2,
                damage: 26,
            },
            Hero {
                health: 1,
                damage: 15,
            },
        ],
        chosen_hero: 17,
        boost_damage: 10,
        max_boosts: 10,
    });
}
