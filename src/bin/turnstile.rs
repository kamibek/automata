use automata::dfa::{DFA};
use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Debug, Hash, Eq, Copy, Clone, PartialEq)]
pub enum State {
    Locked = 0,
    Unlocked = 1,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Coin = 0,
    Push = 1,
}

#[derive(PartialEq)]
pub enum TransitionMode {
    Standard,
    Extended,
}

pub const TRANSITION_TABLE: [[State; 2]; 2] = [
    [State::Unlocked, State::Locked],
    [State::Unlocked, State::Locked]
];

fn main() {
    let delta = |state: State, action: Action| {
        TRANSITION_TABLE[state as usize][action as usize]
    };

    let turnstile = DFA::new(State::Locked, HashSet::from([State::Unlocked]), delta);
    let mut current = turnstile.initial();

    loop {
        print!("\nChoose Transition Mode: [1] Standard [2] Extended [q] Quit: ");
        io::stdout().flush().unwrap();
        
        let mut mode_input = String::new();
        io::stdin().read_line(&mut mode_input).expect("failed to read transition mode!");
        
        let selected_mode = match mode_input.trim() {
            "1" => TransitionMode::Standard,
            "2" => TransitionMode::Extended,
            "q" => break,
            _ => {
                println!("unknown command");
                continue;
            }
        };

        println!("\nInitial state: {:?}", current);
        
        if selected_mode == TransitionMode::Standard {
            loop {
                print!("\nChoose Action: [1] Coin  [2] Push  [q] Back to Mode Selection: ");
                io::stdout().flush().unwrap();

                let mut captured = String::new();
                io::stdin().read_line(&mut captured).expect("failed to read line!");

                let input = captured.trim().to_lowercase();
                let action = match input.as_str() {
                    "1" => Some(Action::Coin),
                    "2" => Some(Action::Push),
                    "q" | "quit" | "exit" => break,
                    _ => {
                        println!("unknown command: {:?}", input);
                        None
                    }
                };

                if let Some(act) = action {
                    let previous = current;
                    current = turnstile.next(previous, act);
                    println!("Action: {:?} \nTransition: {:?} -> {:?}", act, previous, current);
                    if turnstile.is_accepting(&current) {
                        println!("Result: UNLOCKED (Accepting State)");
                    } else {
                        println!("Result: LOCKED");
                    }
                }
                println!("");
            }
        } else {
            print!("\nEnter sequence of actions (1 for Coin, 2 for Push, e.g., 121): ");
            io::stdout().flush().unwrap();

            let mut sequence_input = String::new();
            io::stdin().read_line(&mut sequence_input).expect("failed to read sequence!");
            
            let mut actions = Vec::new();
            for c in sequence_input.trim().chars() {
                match c {
                    '1' => actions.push(Action::Coin),
                    '2' => actions.push(Action::Push),
                    _ => if !c.is_whitespace() { println!("Ignoring invalid character: {:?}", c) },
                }
            }

            if !actions.is_empty() {
                let previous = current;
                current = turnstile.run(actions);
                println!("Sequence processed: {:?} -> {:?}", previous, current);
                if turnstile.is_accepting(&current) {
                    println!("Result: UNLOCKED (Accepting State)");
                } else {
                    println!("Result: LOCKED");
                }
            } else {
                println!("No valid actions provided.");
            }
        }
    }
}
