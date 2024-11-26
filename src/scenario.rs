#[derive(Debug)]
pub struct Scenario {
    pub level: u32,
    pub location: [i32; 2],
    pub story: &'static str,
    pub question: &'static str,
    pub death_story: &'static str,
    pub ignore_story: &'static str,
}

/* Rules for scenarios:
- every location represents a village or a district of a town
- every level 1 scenario is a starting scenario and therefore they must represent a starting situation
- following levels can abstractly reference the toughness, depending on the level, of the decision the player made in the previous level
*/
pub const SCENARIOS: [Scenario; 1] = [
    Scenario {
        level: 1,
        location: [0, 0],
        story: "As the carriage arrives at the fish market, a terrible smell, inspired by rotten eggs soaked in salted urine, wakes you up from a nightmare. After you pay the young coachman a few extra coins, the viscious gaze of a trader nearby surprises you. The old geezer, dressed like the grim reaper, initiates the hymn of the selling torture. Voices and screams follow you around. Some even try to grab you. People have warned you about this fishing village and their greedy trading cult. Yet here you are at the center of the world - X.
        Ideally you'd have forgotten the rumors and warnings about the people here. The literal center of the world must have attracted a lot of unwanted drama and mystery anyway. It's only fair to form your own opinion. However the first impression increases the difficulty of this wish immensely. You pace through the masses. The other tourists share the same disgusted expression. Any decent hurtless trader whose fish smell and look halfway appetizingly will earn your coins and give you a sense of relief before you can finally leave. Suddenly you hear gunshots behind you. The masses get more chaotic and you follow a small group of familiar faces towards the lakeside.
        Near the lake, it has become more calm. Your legs and your ear relax. Your nose on the other hand still urges you to run away. One look at the waters makes you question, if the fish in this village have any chance at all to result in a healthy delicious meal. All you see is an open spillway. How do people live here? Were the gunshots an anomaly or part of the day-to-day life? Maybe the optimism was too high for the reality of this horrific point of interest.
        »Why are you sad?«, an elderly lady asks you. Her wooden cane blocks your way. The bright wide smile seems out of place. You spot the fish stand behind her.
        »I'm not«, you answer abruptly. She laughs.
        »I may be blind. But I cannot unsee the sadness one of our guests must endure. Tell me, what can I do for you?«, she asks you. At least she didn't scream after you. Instead, her gentle voice actually opens you up. You tell her about the decent and fair expectations you had for this village and how in the end the rumors were true. In the meantime a little child sneakily steals a few coins from the register.", 
        question: "Do you tell the lady about the thief?",
        death_story: "  »Ma'am, that boy is stealing your coins!«, you shout. The boy, rattled, drops the coins and runs away. The old lady panics and falls to the ground. When you hold her hand and try to give her a leg-up, all of a sudden she tosses you over her shoulder. She removes a cap from her blind stick, which reveals a sharp end. There's nothing you can do as she stabs you multiple times until you die.",
        ignore_story: "Ignore",
    },
];

pub const LAZY_WORLD_GAME_ENDING: &str = "You wander around in this empty world. You've seen everything and there's nothing left for you. You die from boredom.\nTHE END";

pub const FINAL_ENDING: &str = "Final Ending\nTHE END";
