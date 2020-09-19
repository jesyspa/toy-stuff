use problem::{Hero, Problem};

#[derive(Debug, PartialEq, Eq)]
pub struct Combat {
    pub monster_health: i32,
    pub heroes: Vec<Hero>,
}

#[allow(dead_code)]
fn solution_is_valid(problem: &Problem, boosts: &Vec<usize>) -> bool {
    assert!(problem.chosen_hero < problem.heroes.len());
    match find_hero_with_killing_blow(&apply_boosts(problem, boosts)) {
        Some(i) => problem.chosen_hero == i,
        None => false,
    }
}

fn apply_boosts(problem: &Problem, boosts: &Vec<usize>) -> Combat {
    assert!(boosts.len() <= problem.max_boosts);
    let mut heroes = problem.heroes.clone();
    for boost in boosts {
        heroes[*boost].damage += problem.boost_damage;
    }
    return Combat {
        monster_health: problem.monster_health,
        heroes: heroes,
    };
}

fn find_hero_with_killing_blow(combat: &Combat) -> Option<usize> {
    if combat.heroes.is_empty() {
        return None;
    }
    let mut remaining_monster_health = combat.monster_health;
    let mut first_hero_index = 0;
    let mut first_hero_damage = 0;
    while first_hero_index < combat.heroes.len() {
        for i in first_hero_index..combat.heroes.len() {
            remaining_monster_health -= combat.heroes[i].damage;
            if remaining_monster_health <= 0 {
                return Some(i);
            }
        }
        first_hero_damage += 1;
        if first_hero_damage == combat.heroes[first_hero_index].health {
            first_hero_index += 1;
            first_hero_damage = 0;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_solution() {
        assert!(solution_is_valid(
            &Problem {
                monster_health: 14,
                heroes: vec![Hero {
                    health: 10,
                    damage: 4
                }, Hero {
                    health: 10,
                    damage: 4
                }],
                chosen_hero: 0,
                boost_damage: 4,
                max_boosts: 1,
            },
            &vec![1]
        ))
    }

    #[test]
    fn invalid_solution() {
        assert!(!solution_is_valid(
            &Problem {
                monster_health: 14,
                heroes: vec![Hero {
                    health: 10,
                    damage: 4
                }, Hero {
                    health: 10,
                    damage: 4
                }],
                chosen_hero: 0,
                boost_damage: 4,
                max_boosts: 2,
            },
            &vec![1, 1]
        ))
    }

    #[test]
    fn no_heroes_no_boosts() {
        assert_eq!(
            Combat {
                monster_health: 10,
                heroes: vec![]
            },
            apply_boosts(
                &Problem {
                    monster_health: 10,
                    heroes: vec![],
                    chosen_hero: 0,
                    max_boosts: 0,
                    boost_damage: 0
                },
                &vec![]
            )
        )
    }

    #[test]
    fn two_heroes_no_boosts() {
        assert_eq!(
            Combat {
                monster_health: 10,
                heroes: vec![
                    Hero {
                        health: 10,
                        damage: 5
                    },
                    Hero {
                        health: 10,
                        damage: 10
                    }
                ]
            },
            apply_boosts(
                &Problem {
                    monster_health: 10,
                    heroes: vec![
                        Hero {
                            health: 10,
                            damage: 5
                        },
                        Hero {
                            health: 10,
                            damage: 10
                        }
                    ],
                    chosen_hero: 0,
                    max_boosts: 0,
                    boost_damage: 0
                },
                &vec![]
            )
        )
    }

    #[test]
    fn three_heroes_three_boosts() {
        assert_eq!(
            Combat {
                monster_health: 10,
                heroes: vec![
                    Hero {
                        health: 10,
                        damage: 5
                    },
                    Hero {
                        health: 10,
                        damage: 20
                    },
                    Hero {
                        health: 10,
                        damage: 10
                    },
                ]
            },
            apply_boosts(
                &Problem {
                    monster_health: 10,
                    heroes: vec![
                        Hero {
                            health: 10,
                            damage: 3
                        },
                        Hero {
                            health: 10,
                            damage: 20
                        },
                        Hero {
                            health: 10,
                            damage: 6
                        },
                    ],
                    chosen_hero: 0,
                    max_boosts: 3,
                    boost_damage: 2
                },
                &vec![2, 0, 2]
            )
        )
    }

    #[test]
    #[should_panic]
    fn one_hero_too_many_boosts() {
        apply_boosts(
            &Problem {
                monster_health: 10,
                heroes: vec![Hero {
                    health: 10,
                    damage: 3,
                }],
                chosen_hero: 0,
                max_boosts: 1,
                boost_damage: 2,
            },
            &vec![0, 0, 0],
        );
    }

    #[test]
    #[should_panic]
    fn boost_out_of_bounds() {
        apply_boosts(
            &Problem {
                monster_health: 10,
                heroes: vec![Hero {
                    health: 10,
                    damage: 3,
                }],
                chosen_hero: 0,
                max_boosts: 1,
                boost_damage: 2,
            },
            &vec![2],
        );
    }

    #[test]
    fn empty_combat() {
        assert_eq!(
            None,
            find_hero_with_killing_blow(&Combat {
                monster_health: 10,
                heroes: vec![]
            })
        );
    }

    #[test]
    fn last_hero_dies() {
        assert_eq!(
            None,
            find_hero_with_killing_blow(&Combat {
                monster_health: 10,
                heroes: vec![Hero {
                    health: 2,
                    damage: 1
                }]
            })
        );
    }

    #[test]
    fn single_hero() {
        assert_eq!(
            Some(0),
            find_hero_with_killing_blow(&Combat {
                monster_health: 10,
                heroes: vec![Hero {
                    health: 100,
                    damage: 1
                }]
            })
        );
    }

    #[test]
    fn two_heroes() {
        assert_eq!(
            Some(1),
            find_hero_with_killing_blow(&Combat {
                monster_health: 10,
                heroes: vec![
                    Hero {
                        health: 100,
                        damage: 1
                    },
                    Hero {
                        health: 100,
                        damage: 5
                    }
                ]
            })
        );
    }

    #[test]
    fn two_heroes_one_dies() {
        assert_eq!(
            Some(1),
            find_hero_with_killing_blow(&Combat {
                monster_health: 15,
                heroes: vec![
                    Hero {
                        health: 2,
                        damage: 1
                    },
                    Hero {
                        health: 100,
                        damage: 2
                    }
                ]
            })
        );
    }

    #[test]
    fn three_heroes_one_dies() {
        assert_eq!(
            Some(1),
            find_hero_with_killing_blow(&Combat {
                monster_health: 50,
                heroes: vec![
                    Hero {
                        health: 2,
                        damage: 1,
                    },
                    Hero {
                        health: 100,
                        damage: 10,
                    },
                    Hero {
                        health: 100,
                        damage: 4,
                    }
                ]
            })
        );
    }
}
