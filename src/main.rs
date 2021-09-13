use rand::Rng;
use std::io;

#[derive(PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn get_name(&self) -> &'static str {
        match self {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissors => "Scissors",
        }
    }
    fn get_beats(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
}

fn main() {
    let mut run: u32 = 0;
    let mut player_wins: u32 = 0;
    let mut npc_wins: u32 = 0;

    loop {
        if run >= 3 {
            println!(
                "Rock/r, Paper/p, Scissors/s - Best out of 3 - Run number {} - Score is {}:{} - Continuing due to tie",
                run,
                player_wins,
                npc_wins
            );
        } else {
            println!(
                "Rock/r, Paper/p, Scissors/s - Best out of 3 - Run number {} - Score is {}:{}",
                run, player_wins, npc_wins
            );
        }

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = match choice.trim() {
            "rock" | "r" => Choice::Rock,
            "paper" | "p" => Choice::Paper,
            "scissors" | "scissor" | "s" => Choice::Scissors,
            _ => {
                println!("Input invalid");
                continue;
            }
        };

        run += 1;

        let npc_choice = match rand::thread_rng().gen_range(0..=2) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => unreachable!(),
        };
        println!(
            "You chose {}, computer chose {}",
            choice.get_name(),
            npc_choice.get_name()
        );
        if npc_choice == choice {
            println!("Player and computer tie");
        } else if choice.get_beats() == npc_choice {
            println!(
                "{} beats {} - Player wins",
                choice.get_name(),
                npc_choice.get_name()
            );
            player_wins += 1;
        } else {
            println!(
                "{} beats {} - Player loses",
                npc_choice.get_name(),
                choice.get_name()
            );
            npc_wins += 1;
        }

        if run >= 3 && player_wins != npc_wins {
            if player_wins > npc_wins {
                println!(
                    "Player has won the set with a score of {} to {}",
                    player_wins, npc_wins
                );
            } else {
                println!(
                    "Player has lost the set with a score of {} to {}",
                    player_wins, npc_wins
                );
            }

            break;
        }
    }
}
