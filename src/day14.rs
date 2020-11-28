#[allow(unused_imports)]
use super::prelude::*;
type Input = HashMap<Chem, (u64, Vec<(u64, Chem)>)>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chem(usize);

const FUEL: Chem = Chem(0);
const ORE: Chem = Chem(1);

pub fn input_generator(input: &str) -> Input {
    let mut chem_map = HashMap::new();
    chem_map.insert("FUEL", FUEL);
    chem_map.insert("ORE", ORE);
    let mut idx = 1;

    fn parse_chemical(chem_str: &str) -> (u64, &str) {
        let mut split = chem_str.split(" ");
        let quantity = split.next()
            .expect("Expected chemical quantity")
            .parse()
            .expect("Chemical quantity not an integer");
        let chem_type = split.next()
            .expect("Chemical quantity expected");
        (quantity, chem_type)
    }

    let mut str_to_chem = |chem_str| *chem_map.entry(chem_str).or_insert_with(|| { idx += 1; Chem(idx) });
    
    input.lines().map(|line| {
        let mut split = line.split(" => ");
        let reagents = split.next()
            .expect("Expected reagents")
            .split(", ")
            .map(parse_chemical)
            .map(|(quantity, chem_str)| (quantity, str_to_chem(chem_str)))
            .collect();
        let (product_quantity, product_type) = parse_chemical(split.next().expect("Expected products"));
        let product_chem = str_to_chem(product_type);

        (product_chem, (product_quantity, reagents))
    }).collect()
}

fn produce_fuel(
    fuel_quantity: u64,
    reactions: &HashMap<Chem, (u64, Vec<(u64, Chem)>)>,
    extra_chems: &mut HashMap<Chem, u64>
) -> u64 {
    let mut need_chems = VecDeque::new();
    need_chems.push_back((FUEL, fuel_quantity));
    let mut required_ore = 0;

    loop {
        match need_chems.pop_front() {
            Some((need, mut quantity)) if need == ORE => {
                let extra_ore = extra_chems.entry(ORE).or_insert(0);
                let extra_used = std::cmp::min(quantity, *extra_ore);
                quantity -= extra_used;
                *extra_ore -= extra_used;
                required_ore += quantity;
            },
            Some((need, mut quantity)) => {
                let extra = extra_chems.entry(need.clone()).or_insert(0);
                let extra_used = std::cmp::min(quantity, *extra);
                quantity -= extra_used;
                *extra -= extra_used;
                if quantity > 0 {
                    let (reaction_quantity, reagents) = reactions.get(&need)
                        .expect("Missing reaction for a reagent");
                    let reaction_multiplier = (quantity - 1) / reaction_quantity + 1;
                    *extra = reaction_quantity * reaction_multiplier - quantity;
                    for &(reagent_quantity, reagent_chem) in reagents {
                        need_chems.push_back((reagent_chem, reagent_quantity * reaction_multiplier));
                    }
                }
            },
            None => return required_ore,
        }
    }
}

pub fn part1(input: &Input) -> u64 {
    let reactions = input;
    produce_fuel(1, reactions, &mut HashMap::new())
}

pub fn part2(input: &Input) -> u64 {
    let reactions = input;
    let mut extra = HashMap::new();
    
    let ore_per_fuel = produce_fuel(1, reactions, &mut extra);
    let mut produced_fuel = 1;
    // Performance fix if input contains a reaction that produces more than 1 FUEL
    if extra.contains_key(&FUEL) {
        produced_fuel += extra.remove(&FUEL).unwrap();
    }
    let produced_fuel_multiplier = produced_fuel;

    *extra.entry(ORE).or_insert(0) = 1_000_000_000_000 - ore_per_fuel;

    loop {
        let produceable_fuel = std::cmp::max(1, extra.get(&ORE).unwrap() / ore_per_fuel * produced_fuel_multiplier);
        let required_ore = produce_fuel(produceable_fuel, &reactions, &mut extra);
        if required_ore != 0 {
            return produced_fuel
        }
        produced_fuel += produceable_fuel;
    }
}
