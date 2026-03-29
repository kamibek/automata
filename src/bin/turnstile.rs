use automata::dfa::{DFA};
use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Debug, Hash, Eq, Copy, Clone, PartialEq)]
pub enum State{
    Locked = 0,
    Unlocked = 1,
}

#[derive(Debug, Hash,Copy, Clone, Eq, PartialEq)]
pub enum Action{
    Coin = 0,
    Push = 1,
}

pub const TRANSITION_TABLE: [[State; 2]; 2] = [
    [State::Unlocked, State::Locked],
    [State::Unlocked, State::Locked]
];

fn main(){
    let delta = |state: State, action: Action|{
        TRANSITION_TABLE[state as usize][action as usize]
    };

    let turnstile = DFA::new(State::Locked, HashSet::from([State::Unlocked]), delta);
    let mut current = turnstile.initial();
    println!("\nInitial state: {:?}", current);

    loop{
        print!("Choose Action: [1] Push  [2] Coin  [q] Quit: ");
            io::stdout().flush().unwrap();

        let mut captured: String = String::new();
        io::stdin().read_line(&mut captured).expect("failed to read line!");

        let input: String = captured.trim().to_lowercase();

        let action = match input.as_str(){
            "1" => Some(Action::Push),
            "2" => Some(Action::Coin),
            "q" | "quit" | "exit" => break,
            _ => {
                println!("unknown command: {:?}",input);
                None
            }
        };

        if let Some(act) = action{
            let previous = current;
            current = turnstile.next(previous, act);
            println!("Action: {:?} \nTransition: {:?} -> {:?}", act, previous, current);
                if turnstile.is_accepting(&current) {
                    println!("Result: UNLOCKED (Accepting State)");
                } else {
                    println!("Result: LOCKED");
                }
        }
        println!("\n");
    }
}


