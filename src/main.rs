#[derive(Debug)]
struct Scenario {
    level: u32,
    location: [u32; 2],
    story: &'static str,
    death_story: &'static str,
}

const SCENARIOS: [Scenario; 1] = [Scenario {
    level: 1,
    location: [0, 0],
    story: "",
    death_story: "",
}];

fn main() {}
