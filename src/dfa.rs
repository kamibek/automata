use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct DFA<TState, TAlphabet, FDelta>
where
    FDelta: Fn(TState, TAlphabet) -> TState,
    TState: Eq + Copy + Clone + Hash
{
    _initial: TState,
    _accepting: HashSet<TState>,
    _delta: FDelta,
    _marker: PhantomData<TAlphabet>
}

impl<TState, TAlphabet, FDelta> DFA<TState, TAlphabet, FDelta>
where
    FDelta: Fn(TState, TAlphabet) -> TState,
    TState: Eq + Copy + Clone + Hash
{
    pub fn new(initial: TState, accepting: HashSet<TState>, delta: FDelta) -> Self{
        DFA{
            _initial: initial,
            _accepting: accepting,
            _delta: delta,
            _marker: PhantomData::<TAlphabet>
        }
    }

    pub fn initial(&self) -> TState{ self._initial}
}
