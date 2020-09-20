extern crate rust_monster;
use rust_monster::problem::{Hero, Problem};
use rust_monster::solution::solve;

// Solution to
// https://gist.github.com/1Computer1/125ab56958ba15ac625d78a5a08df9e0
// We make the following modifications:
// * Initial monster turn is skipped.
fn main() {
    let problem = Problem {
        monster_health: 10,
        heroes: vec![Hero {
            health: 3,
            damage: 4,
        }],
        chosen_hero: 0,
        boost_damage: 1,
        max_boosts: 1,
    };
    match solve(&problem) {
        Some(solution) => {
            println!("success");
            for i in solution {
                print!("{} ", i);
            }
            println!();
        }
        None => {
            println!("failure");
        }
    }
}
