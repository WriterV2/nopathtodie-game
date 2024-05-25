#[derive(Debug)]
pub struct Scenario {
    pub level: u32,
    pub location: [i32; 2],
    pub story: &'static str,
    pub question: &'static str,
    pub death_story: &'static str,
    pub ignore_story: &'static str,
}

pub const SCENARIOS: [Scenario; 4] = [
    Scenario {
        level: 1,
        location: [0, 0],
        story: "A",
        question: "?",
        death_story: "Death",
        ignore_story: "Ignore",
    },
    Scenario {
        level: 1,
        location: [1, 0],
        question: "?",
        story: "B",
        death_story: "Death",
        ignore_story: "Ignore",
    },
    Scenario {
        level: 2,
        location: [-1, 1],
        story: "C",
        question: "?",
        death_story: "Death",
        ignore_story: "Ignore",
    },
    Scenario {
        level: 3,
        location: [-1, 0],
        story: "D",
        question: "?",
        death_story: "Death",
        ignore_story: "Ignore",
    },
];

pub const LAZY_WORLD_GAME_ENDING: &str = "You wander around in this empty world. You've seen everything and there's nothing left for you. You die from boredom.\nTHE END";

pub const FINAL_ENDING: &str = "Final Ending\nTHE END";
