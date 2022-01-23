use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub type Key = [u8; 16];

#[derive(Debug, Clone)]
pub struct SymmetricKeySpace {
    pub first: u64,
    pub last: u64,
}

#[derive(Debug, Clone)]
pub struct UnstructuredKeySpace {
    pub first: u128,
    pub last: u128,
}

fn key_from_numeric_half_key(half_key: u64) -> Key {
    half_key.to_be_bytes().repeat(2).try_into().unwrap()
}

type KeyFromNumericHalfKey = fn(u64) -> Key;

fn key_from_numeric_key(key: u128) -> Key {
    key.to_be_bytes()
}

type KeyFromNumericKey = fn(u128) -> Key;

impl IntoParallelIterator for SymmetricKeySpace {
    type Item = Key;

    type Iter = rayon::iter::Map<rayon::range_inclusive::Iter<u64>, KeyFromNumericHalfKey>;

    fn into_par_iter(self) -> Self::Iter {
        (self.first..=self.last)
            .into_par_iter()
            .map(key_from_numeric_half_key)
    }
}

impl IntoParallelIterator for UnstructuredKeySpace {
    type Item = Key;

    type Iter = rayon::iter::Map<rayon::range_inclusive::Iter<u128>, KeyFromNumericKey>;

    fn into_par_iter(self) -> Self::Iter {
        (self.first..=self.last)
            .into_par_iter()
            .map(key_from_numeric_key)
    }
}
