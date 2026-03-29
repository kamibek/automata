use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;

//  5-Tuple Definition of NFA
//  M = (Q, ∑, δ, q₀, F)
//  δ : Q x (∑ ∪ {ε}) ⇾ P(Q)
//  Q = TState
//  ∑ = TAlphabet
//  δ = FDelta
//  () = ε
//  (∑ ∪ {ε}) = Optional<TAlphabet>

pub struct NFA<TState, TAlphabet, FDelta>
where
    FDelta: Fn(TState, Option<TAlphabet>) -> Vec<TState>,
    TState: Eq + Copy + Clone + Hash,
    TAlphabet: Eq + Copy + Clone
{
    _initial: TState,
    _accepting: HashSet<TState>,
    _delta: FDelta,
    _marker: PhantomData<TAlphabet>
}

impl<TState, TAlphabet, FDelta> NFA<TState, TAlphabet, FDelta>
where
    FDelta: Fn(TState, Option<TAlphabet>) -> Vec<TState>,
    TState: Eq + Copy + Clone + Hash,
    TAlphabet: Eq + Copy + Clone,
{
    pub fn new(initial: TState, accepting: HashSet<TState>, delta: FDelta) -> Self{
        NFA{
            _initial: initial,
            _accepting: accepting,
            _delta: delta,
            _marker: PhantomData::<TAlphabet>
        }
    }

    pub fn initial(&self) -> TState{ self._initial }

    pub fn accepting(&self) -> &HashSet<TState>{ &self._accepting }

    pub fn is_accepting(&self, state: TState) -> bool{ self._accepting.contains(&state) }

    pub fn next(&self, state: TState, symbol: TAlphabet) -> Vec<TState>{
        (self._delta)(state, Some(symbol))
    }

    pub fn run(&self, string: Vec<TAlphabet>) -> Vec<TState>{
        todo!()
    }
}
