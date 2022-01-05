use clap::Parser;
use itertools::izip;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

/// Program to calculate winning odds in the risk board game.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Number of armies the attacker has
    #[clap(short, long, default_value_t = 12)]
    attacker_armies: usize,

    /// Number of armies the defender has
    #[clap(short, long, default_value_t = 8)]
    defender_armies: usize,

    /// How many games will be simulated
    #[clap(short, long, default_value_t = 50000)]
    number_of_simulations: usize,

    /// Cancel the attack if the attacker has fewer or equal than this number of armies
    #[clap(short, long, default_value_t = 1)]
    cancel_attack: usize,

    /// Print detailed battle statistics to the screen. Will be deactivated for large number of simulations.
    #[clap(short, long)]
    verbose: bool,
}

fn roll_die(times: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let die_range = Uniform::new_inclusive(1, 6);
    let roll_die = (&mut rng).sample_iter(die_range);
    roll_die.take(times).collect()
}

//TODO: average rounds, calculate execution time

fn main() {
    let args = Args::parse();

    let mut do_logging = args.verbose;
    let number_of_simulations = args.number_of_simulations;
    if number_of_simulations > 10 && do_logging {
        println!("Deactivating logging due to large number of installations.");
        do_logging = false;
    }
    let mut attacker_wins = 0;
    let mut defender_wins = 0;
    let input_attacker_armies = args.attacker_armies;
    let input_defender_armies = args.defender_armies;
    let cancel_attack_threshold = args.cancel_attack;
    for game_idx in 0..number_of_simulations {
        if do_logging {
            println!("game {}", game_idx);
        }
        let mut attacker_armies = input_attacker_armies;
        let mut defender_armies = input_defender_armies;
        while attacker_armies >= cancel_attack_threshold && defender_armies > 0 {
            if do_logging {
                println!("--------------------");
            }
            let number_dice_attacker = std::cmp::min(attacker_armies - 1, 3);
            let number_dice_defender = std::cmp::min(defender_armies, 3);
            if do_logging {
                println!(
                    "Attacker rolls {} dice, defender rolls {} dice",
                    number_dice_attacker, number_dice_defender
                );
            }
            let mut attacker_dice = roll_die(number_dice_attacker);
            attacker_dice.sort_unstable();
            attacker_dice.reverse();
            let mut defender_dice = roll_die(number_dice_defender);
            defender_dice.sort_unstable();
            defender_dice.reverse();
            for (attacker_roll, defender_roll) in izip!(&attacker_dice, &defender_dice) {
                if do_logging {
                    println!(
                        "attacker rolled: {}  defender rolled: {}",
                        attacker_roll, defender_roll
                    );
                }
                if (defender_roll - attacker_roll) >= 0 {
                    attacker_armies -= 1;
                } else {
                    defender_armies -= 1;
                }
            }
            if do_logging {
                println!(
                    "Attacker has {} armies, defender has {} armies",
                    attacker_armies, defender_armies
                );
            }
        }
        if do_logging {
            println!("--------------------");
        }
        if attacker_armies <= cancel_attack_threshold && defender_armies > 0 {
            if do_logging {
                println!(
                    "defender won because attacker has {} armies which is equal or below {}",
                    attacker_armies, cancel_attack_threshold
                )
            };
            defender_wins += 1;
        } else {
            if do_logging {
                println!("attacker won");
            }
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
