use std::io::{self, prelude::*};

#[derive(Debug)]
struct Elf {
    food_carried: Vec<Food>,
}

#[derive(Debug)]
struct Food {
    calories: u32,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            food_carried: Vec::new(),
        }
    }

    fn count_calories(&self) -> u32 {
        self.food_carried.iter().map(|f| f.calories).sum()
    }

    fn carry_food(&mut self, food: Food) {
        self.food_carried.push(food);
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut elves = Vec::new();

    lines.split(|l| l.is_empty()).for_each(|elf_data| {
        let mut elf = Elf::new();
        for l in elf_data {
            let calories = l.parse::<u32>().unwrap();
            elf.carry_food(Food { calories });
        }
        elves.push(elf);
    });

    elves.sort_by(|a, b| b.count_calories().cmp(&a.count_calories()));

    // println!(
    //     "{:?}",
    //     elves.iter().map(|e| e.count_calories()).collect::<Vec<_>>()
    // );

    // let fattest_elf = elves.iter().max_by_key(|e| e.count_calories()).unwrap();
    let fattest_elf = &elves[0];

    println!(
        "Calories carried by the most calorific elf is {}.",
        fattest_elf.count_calories()
    );

    let fattest_three_elves = &elves[0..3];

    println!(
        "Calories carried by the three most calorific elves are {}.",
        fattest_three_elves
            .iter()
            .map(Elf::count_calories)
            .sum::<u32>()
    );
}
