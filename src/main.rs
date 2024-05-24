use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Scenario {
    level: u32,
    location: [i32; 2],
    story: &'static str,
    death_story: &'static str,
}

const SCENARIOS: [Scenario; 3] = [
    Scenario {
        level: 1,
        location: [0, 0],
        story: "A",
        death_story: "",
    },
    Scenario {
        level: 1,
        location: [-1, 1],
        story: "B",
        death_story: "",
    },
    Scenario {
        level: 1,
        location: [1, 1],
        story: "C",
        death_story: "",
    },
];

#[derive(Debug)]
struct Game {
    current_level: u32,
    current_location: [i32; 2],
}

impl Default for Game {
    fn default() -> Self {
        let mut rng = thread_rng();
        let starting_point = |is_x: bool, is_lowest: bool| {
            let n = if is_x { 0 } else { 1 };
            if let Some(scenario) = SCENARIOS.iter().filter(|x| x.level == 1).min_by(|a, b| {
                if is_lowest {
                    a.location[n].cmp(&b.location[n])
                } else {
                    b.location[n].cmp(&a.location[n])
                }
            }) {
                scenario.location[n]
            } else {
                0
            }
        };

        Game {
            current_level: 1,
            current_location: [
                rng.gen_range(starting_point(true, true)..=starting_point(true, false)),
                rng.gen_range(starting_point(false, true)..=starting_point(false, false)),
            ],
        }
    }
}

impl Game {
    fn run(mut self) {
        let mut rng = thread_rng();
        let scenarios = SCENARIOS.iter().filter(|scenario| {
            scenario.level == self.current_level
                && scenario.location[0] <= self.current_location[0] + 1
                && scenario.location[0] >= self.current_location[0] - 1
                && scenario.location[1] <= self.current_location[1] + 1
                && scenario.location[1] >= self.current_location[1] - 1
        });
        let scenario = scenarios.choose(&mut rng).expect("No more scenarios left");
        self.current_location = scenario.location;
        self.current_level += 1;
        println!("{}", scenario.story);
    }
}

fn main() {
    Game::default().run();
}
