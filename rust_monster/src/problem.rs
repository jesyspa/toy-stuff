#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hero {
    pub health: usize,
    pub damage: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    pub monster_health: i32,
    pub heroes: Vec<Hero>,
    pub chosen_hero: usize,
    pub boost_damage: i32,
    pub max_boosts: usize,
}
