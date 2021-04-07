extern crate rust_monster;
use rust_monster::problem::{Hero, Problem};
use rust_monster::solution::solve;
use rust_monster::simulation::solution_is_valid;

// Solution to
// https://gist.github.com/1Computer1/125ab56958ba15ac625d78a5a08df9e0
// We make the following modifications:
// * Initial monster turn is skipped.
fn main() {
    let problem = Problem {
        monster_health: 856867849,
        heroes: vec![
            Hero {
                health: 29,
                damage: 1910,
            },
            Hero {
                health: 2112,
                damage: 195,
            },
            Hero {
                health: 43880,
                damage: 16,
            },
            Hero {
                health: 1,
                damage: 18586,
            },
        ],
        chosen_hero: 2,
        boost_damage: 1,
        max_boosts: 20,
    };
    match solve(&problem) {
        Some(solution) => {
            println!("success");
            for i in &solution {
                print!("{} ", i);
            }
            println!();
            println!("Solution is valid: {}", solution_is_valid(&problem, &solution));
        }
        None => {
            println!("failure");
        }
    }
    println!("Trivial solution: {}", solution_is_valid(&problem, &[]));
}
