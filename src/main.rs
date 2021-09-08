use rand::Rng;
use std::io;

fn get_name(choice: u32) -> &'static str {
    match choice {
        0 => "Rock",
        1 => "Paper",
        2 => "Scissors",
        _ => unreachable!(),
    }
}

fn get_num(choice: String) -> Result<u32, String> {
    let choice = choice.trim();
    match choice.parse() {
        Ok(num) => match num {
            0..=2 => Ok(num),
            _ => Err("Invalid input".to_string()),
        },
        Err(_) => match choice {
            "rock" | "r" => Ok(0),
            "paper" | "p" => Ok(1),
            "scissors" | "scissor" | "s" => Ok(2),
            _ => Err("Invalid input".to_string()),
        },
    }
}

fn get_strength(choice: u32) -> u32 {
    match choice {
        0 => 2,
        1 => 0,
        2 => 1,
        _ => unreachable!(),
    }
}

fn main() {
    let mut run: u32 = 0;
    let mut player_wins: u32 = 0;
    let mut npc_wins: u32 = 0;

    loop {
        if run >= 3 {
            println!(
                "Rock/r/0, Paper/p/1, Scissors/s/2 - Best out of 3 - Run number {} - Score is {}:{} - Continuing due to tie",
                run,
                player_wins,
                npc_wins
            );
        } else {
            println!(
                "Rock/r/0, Paper/p/1, Scissors/s/2 - Best out of 3 - Run number {} - Score is {}:{}",
                run,
                player_wins,
                npc_wins
            );
        }

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match get_num(choice) {
            Ok(num) => num,
            Err(_) => {
                println!("Input invalid");
                continue;
            }
        };

        run += 1;

        let npc_choice = rand::thread_rng().gen_range(0..=2);
        println!(
            "You chose {}, computer chose {}",
            get_name(choice),
            get_name(npc_choice)
        );

        if npc_choice == choice {
            println!("Player and computer tie");
        } else if choice == get_strength(npc_choice) {
            println!(
                "{} beats {} - Player loses",
                get_name(npc_choice),
                get_name(choice)
            );
            npc_wins += 1;
        } else {
            println!(
                "{} beats {} - Player wins",
                get_name(choice),
                get_name(npc_choice)
            );
            player_wins += 1;
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
