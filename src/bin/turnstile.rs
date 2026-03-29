use automata::dfa::{DFA};
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, Copy, Clone, PartialEq)]
pub enum State{
    Locked = 0,
    Unlocked = 1,
}

#[derive(Debug, Hash, Eq, PartialEq)]
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
    let current = turnstile.initial();
    println!("{:?}", current);
}


