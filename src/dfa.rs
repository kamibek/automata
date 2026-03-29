use std::collections::HashSet;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct DFA<TState, TAlphabet, FDelta>
where 
    FDelta: Fn(TState, TAlphabet) -> TState,
    TState: Eq + Hash
{
    _initial_state: TState,
    _accepting: HashSet<TState>,
    _delta: FDelta,
    _marker: PhantomData<TAlphabet>
}

impl<TState, TAlphabet, FDelta> DFA<TState, TAlphabet, FDelta>
where 
    FDelta: Fn(TState, TAlphabet) -> TState,
    TState: Eq + Hash
{
    pub fn new(q0: TState, accepting: HashSet<TState>, delta: FDelta) -> Self{
        DFA{
            _initial_state: q0,
            _accepting: accepting,
            _delta: delta,
            _marker: PhantomData<TAlphabet>
        }
    }

    pub fn initial(&self) -> TState{self._initial_state;}

    pub fn accepting(&self) -> HashSet{self._accepting;}

    pub fn step(state: TState, symbol: TAlphabet) -> TState{
        (self._delta)(state, symbol)
    }

    pub fn run(string: [TAlphabet]) -> TState{
        let mut current: TState = self._initial_state;
        for symbol in string{
            current = (self._delta)(current, symbol);
        }
        current
    }
}
