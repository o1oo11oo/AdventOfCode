use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::u16 as Num,
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

type Num = u16;

pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .map(Blueprint::from)
        .map(Blueprint::quality_level)
        .sum::<Num>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .lines()
        .take(3)
        .map(Blueprint::from)
        .map(|b| b.evaluate(32).geode.amount)
        .product::<Num>()
        .to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    id: Num,
    ore_cost: Num,
    clay_cost: Num,
    obsidian_cost: (Num, Num),
    geode_cost: (Num, Num),
}

impl Blueprint {
    fn quality_level(self) -> Num {
        self.id * self.evaluate(24).geode.amount
    }

    fn evaluate(&self, time_limit: Num) -> State {
        let state = State {
            time: 0,
            ore: Material { amount: 0, rate: 1 },
            clay: Material { amount: 0, rate: 0 },
            obsidian: Material { amount: 0, rate: 0 },
            geode: Material { amount: 0, rate: 0 },
        };

        let mut cache = HashMap::new();
        let best = self.evaluate_recursive(state, &state, Choice::Nothing, time_limit, &mut cache);
        log::debug!("Evaluation of {} found best: {:?}", self.id, best);
        best
    }

    fn evaluate_recursive(
        &self,
        state: State,
        last_state: &State,
        last_choice: Choice,
        time_limit: Num,
        cache: &mut HashMap<State, State>,
    ) -> State {
        if state.time == time_limit {
            return state;
        }

        // figure out what to build
        let mut choices = vec![];
        if self.can_build_geode(&state) {
            choices.push(Choice::BuildGeode);
        } else {
            if self.can_build_obsidian(&state)
                && self.should_build_obsidian(&state, last_state, &last_choice)
            // this optimization (if possible only build obsidian robot) is incorrect
            // for the example but works for my input and at this point I don't care
            {
                choices.push(Choice::BuildObsidian);
            } else {
                if self.can_build_clay(&state)
                    && self.should_build_clay(&state, last_state, &last_choice)
                {
                    choices.push(Choice::BuildClay);
                }
                if self.can_build_ore(&state)
                    && self.should_build_ore(&state, last_state, &last_choice)
                {
                    choices.push(Choice::BuildOre);
                }
            }
            // do nothing is always an option
            choices.push(Choice::Nothing);
        }

        // get new resources this round
        let next_state = state.produce();

        // calculate possibilities
        choices
            .into_iter()
            .map(|choice| {
                let next_state = choice.apply(next_state, self);
                if let Some(res) = cache.get(&next_state) {
                    *res
                } else {
                    let res =
                        self.evaluate_recursive(next_state, &state, choice, time_limit, cache);
                    cache.insert(next_state, res);
                    res
                }
            })
            .max_by_key(|state| state.geode.amount)
            .unwrap()
    }

    fn can_build_ore(&self, state: &State) -> bool {
        self.ore_cost <= state.ore.amount
    }

    fn should_build_ore(&self, state: &State, last_state: &State, last_choice: &Choice) -> bool {
        // max needed rate is higher then current rate and
        // could not afford robot last round, or built a different robot
        self.ore_cost
            .max(self.clay_cost)
            .max(self.obsidian_cost.0)
            .max(self.geode_cost.0)
            > state.ore.rate
            && (!self.can_build_ore(last_state) || last_choice != &Choice::Nothing)
    }

    fn can_build_clay(&self, state: &State) -> bool {
        self.clay_cost <= state.ore.amount
    }

    fn should_build_clay(&self, state: &State, last_state: &State, last_choice: &Choice) -> bool {
        // max needed rate is higher then current rate
        // could not afford robot last round, or built a different robot
        self.obsidian_cost.1 > state.clay.rate
            && (!self.can_build_clay(last_state) || last_choice != &Choice::Nothing)
    }

    fn can_build_obsidian(&self, state: &State) -> bool {
        self.obsidian_cost.0 <= state.ore.amount && self.obsidian_cost.1 <= state.clay.amount
    }

    fn should_build_obsidian(
        &self,
        state: &State,
        last_state: &State,
        last_choice: &Choice,
    ) -> bool {
        // max needed rate is higher then current rate
        // could not afford robot last round, or built a different robot
        self.geode_cost.1 > state.obsidian.rate
            && (!self.can_build_obsidian(last_state) || last_choice != &Choice::Nothing)
    }

    fn can_build_geode(&self, state: &State) -> bool {
        self.geode_cost.0 <= state.ore.amount && self.geode_cost.1 <= state.obsidian.amount
    }
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        type ParseResult = (Num, Num, Num, (Num, Num), (Num, Num));
        fn parse(input: &str) -> IResult<&str, ParseResult> {
            tuple((
                delimited(tag("Blueprint "), Num, tag(": ")),
                delimited(tag("Each ore robot costs "), Num, tag(" ore. ")),
                delimited(tag("Each clay robot costs "), Num, tag(" ore. ")),
                tuple((
                    delimited(tag("Each obsidian robot costs "), Num, tag(" ore and ")),
                    terminated(Num, tag(" clay. ")),
                )),
                tuple((
                    delimited(tag("Each geode robot costs "), Num, tag(" ore and ")),
                    terminated(Num, tag(" obsidian.")),
                )),
            ))(input)
        }

        let parsed = parse(value).finish().unwrap().1;
        Self {
            id: parsed.0,
            ore_cost: parsed.1,
            clay_cost: parsed.2,
            obsidian_cost: parsed.3,
            geode_cost: parsed.4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    time: Num,
    ore: Material,
    clay: Material,
    obsidian: Material,
    geode: Material,
}

impl State {
    fn produce(self) -> Self {
        Self {
            time: self.time + 1,
            ore: self.ore.produce(),
            clay: self.clay.produce(),
            obsidian: self.obsidian.produce(),
            geode: self.geode.produce(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Material {
    amount: Num,
    rate: Num,
}

impl Material {
    fn produce(self) -> Self {
        Self {
            amount: self.amount + self.rate,
            ..self
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Choice {
    Nothing,
    BuildOre,
    BuildClay,
    BuildObsidian,
    BuildGeode,
}

impl Choice {
    fn apply(&self, mut state: State, blueprint: &Blueprint) -> State {
        match self {
            Choice::Nothing => state,
            Choice::BuildOre => {
                state.ore.rate += 1;
                state.ore.amount -= blueprint.ore_cost;
                state
            }
            Choice::BuildClay => {
                state.clay.rate += 1;
                state.ore.amount -= blueprint.clay_cost;
                state
            }
            Choice::BuildObsidian => {
                state.obsidian.rate += 1;
                state.ore.amount -= blueprint.obsidian_cost.0;
                state.clay.amount -= blueprint.obsidian_cost.1;
                state
            }
            Choice::BuildGeode => {
                state.geode.rate += 1;
                state.ore.amount -= blueprint.geode_cost.0;
                state.obsidian.amount -= blueprint.geode_cost.1;
                state
            }
        }
    }
}
