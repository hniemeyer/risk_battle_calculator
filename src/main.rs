use itertools::izip;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

fn roll_die(times: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let die_range = Uniform::new_inclusive(1, 6);
    let roll_die = (&mut rng).sample_iter(die_range);
    roll_die.take(times).collect()
}

fn main() {
    let number_of_simulations = 50000;
    let mut attacker_wins = 0;
    let mut defender_wins = 0;
    let input_attacker_armies = 12;
    let input_defender_armies = 22;
    for game_idx in 1..number_of_simulations {
        println!("game {}", game_idx);
        let mut attacker_armies = input_attacker_armies;
        let mut defender_armies = input_defender_armies;
        while attacker_armies > 1 && defender_armies > 0 {
            println!("--------------------");
            let number_dice_attacker = std::cmp::min(attacker_armies - 1, 3);
            let number_dice_defender = std::cmp::min(defender_armies, 3);
            println!(
                "Attacker rolls {} dice, defender rolls {} dice",
                number_dice_attacker, number_dice_defender
            );
            let mut attacker_dice = roll_die(number_dice_attacker);
            attacker_dice.sort_unstable();
            attacker_dice.reverse();
            let mut defender_dice = roll_die(number_dice_defender);
            defender_dice.sort_unstable();
            defender_dice.reverse();
            for (attacker_roll, defender_roll) in izip!(&attacker_dice, &defender_dice) {
                println!(
                    "attacker rolled: {}  defender rolled: {}",
                    attacker_roll, defender_roll
                );
                if (defender_roll - attacker_roll) >= 0 {
                    attacker_armies -= 1;
                } else {
                    defender_armies -= 1;
                }
            }
            println!(
                "Attacker has {} armies, defender has {} armies",
                attacker_armies, defender_armies
            );
        }
        println!("--------------------");
        if attacker_armies == 1 {
            println!("defender won");
            defender_wins += 1;
        }
        if defender_armies == 0 {
            println!("attacker won");
            attacker_wins += 1;
        }
    }
    println!(
        "Out of {} games, the attacker won {} times and the defender {} times if the atacker has {} and the defender {} armies",
        number_of_simulations, attacker_wins, defender_wins, input_attacker_armies, input_defender_armies
    );
    let win_chance_attacker = (attacker_wins as f64) / (number_of_simulations as f64) * 100.0;
    println!("Win chance of attacker: {} percent", win_chance_attacker);
}
