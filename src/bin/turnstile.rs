use std::env;
use automata::dfa;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum State{
    Locked,
    Unlocked,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Action{
    Coin,
    Push,
}

pub fn delta(state: State, action: Action) -> State{
    match (state, action) {
        (State::Locked, Action::Push) => State::Locked,
        (State::Locked, Action::Coin) => State::Unlocked,
        (State::Unlocked, Action::Push) => State::Locked,
        (State::Unlocked, Action::Coin) => State::Unlocked,
        _ => {
            panic!("something went wrong")
        }
    }
}
fn main(){
    let turnstile = DFA::new(State::Locked, HashSet::from([State::Unlocked]), delta);
    let current = turnstile.initial();
    println!("{:?}", current);
}


