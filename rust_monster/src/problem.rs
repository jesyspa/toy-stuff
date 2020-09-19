#[derive(Debug, PartialEq, Eq)]
pub struct Hero {
    pub health: i32,
    pub damage: i32,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    pub monster_health: i32,
    pub monster_damage: i32,
    pub heroes: Vec<Hero>,
    pub chosen_hero: usize,
    pub boost_damage: i32,
    pub max_boosts: usize,
}
