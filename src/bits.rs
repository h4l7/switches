#![allow(dead_code)]

use itertools::{Combinations, Itertools};
use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    error::Error,
    fmt,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Not, Shl,
        ShlAssign, Shr, ShrAssign,
    },
    str::FromStr,
};

#[derive(Debug)]
pub struct Zeroes<const N: usize> {
    bits: Bits<N>,
    cursor: usize,
}

impl<const N: usize> Zeroes<N> {
    fn new(bits: &Bits<N>) -> Self {
        Self {
            bits: *bits,
            cursor: 0,
        }
    }
}

impl<const N: usize> Iterator for Zeroes<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.bits.len() {
            let i = self.cursor;

            if !self.bits[i] {
                self.cursor += 1;
                return Some(i);
            }

            self.cursor += 1;
        }

        None
    }
}

#[derive(Debug)]
pub struct Ones<const N: usize> {
    inner: Zeroes<N>,
}

impl<const N: usize> Ones<N> {
    fn new(bits: &Bits<N>) -> Self {
        Self {
            inner: Zeroes::new(&!*bits),
        }
    }
}

impl<const N: usize> Iterator for Ones<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Debug)]
pub struct UpperShadow<const N: usize> {
    inner: Converge<N>,
}

impl<const N: usize> UpperShadow<N> {
    fn new(origin: &Bits<N>) -> Self {
        Self {
            inner: Converge::new(origin, &Bits::new(true)).unwrap(),
        }
    }
}

impl<const N: usize> Iterator for UpperShadow<N> {
    type Item = Bits<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Debug)]
pub struct LowerShadow<const N: usize> {
    inner: UpperShadow<N>,
}

impl<const N: usize> LowerShadow<N> {
    fn new(origin: &Bits<N>) -> Self {
        Self {
            inner: UpperShadow::new(&!*origin),
        }
    }
}

impl<const N: usize> Iterator for LowerShadow<N> {
    type Item = Bits<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|next| !next)
    }
}

#[derive(Debug)]
pub struct IncomparableError;

impl fmt::Display for IncomparableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IncomparableError")
    }
}

impl Error for IncomparableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct Midpoints<const N: usize> {
    origin: Bits<N>,
    combs_lower: Combinations<Ones<N>>,
    combs_upper: Combinations<Ones<N>>,
}

impl<const N: usize> Midpoints<N> {
    fn new(start: &Bits<N>, end: &Bits<N>) -> Result<Self, IncomparableError> {
        if start.partial_cmp(end).is_none() {
            return Err(IncomparableError);
        }

        let origin: Bits<N>;
        let target: Bits<N>;

        if *start > *end {
            origin = *end;
            target = *start;
        } else {
            origin = *start;
            target = *end;
        }

        let diff = origin ^ target;
        let size = diff.count_ones();
        let combs_lower: Combinations<Ones<N>>;
        let combs_upper: Combinations<Ones<N>>;

        if size % 2 == 0 {
            let count = size / 2;

            combs_lower = diff.ones().combinations(count);
            combs_upper = diff.ones().combinations(0);
        } else {
            let count_lower = size / 2;
            let count_upper = (size / 2) + 1;

            combs_lower = diff.ones().combinations(count_lower);
            combs_upper = diff.ones().combinations(count_upper);
        }

        Ok(Self {
            origin,
            combs_lower,
            combs_upper,
        })
    }
}

impl<const N: usize> Iterator for Midpoints<N> {
    type Item = Bits<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ones) = self.combs_lower.next() {
            let mut next = self.origin;

            for i in ones {
                next[i] = true;
            }

            return Some(next);
        } else if let Some(ones) = self.combs_upper.next() {
            if ones.is_empty() {
                return None;
            }

            let mut next = self.origin;

            for i in ones {
                next[i] = true;
            }

            return Some(next);
        }

        None
    }
}

#[derive(Debug)]
pub struct Converge<const N: usize> {
    cursor: Bits<N>,
    origin: Bits<N>,
    target: Bits<N>,
    seen: HashSet<Bits<N>>,
    pending: VecDeque<Bits<N>>,
    reversed: bool,
    initialized: bool,
}

impl<const N: usize> Converge<N> {
    fn new(start: &Bits<N>, end: &Bits<N>) -> Result<Self, IncomparableError> {
        if start.partial_cmp(end).is_none() {
            return Err(IncomparableError);
        }

        let origin = *start;
        let target = *end;
        let cursor = origin;
        let seen = HashSet::<Bits<N>>::new();
        let mut pending = VecDeque::<Bits<N>>::new();
        let reversed = origin > target;
        let initialized = false;

        let diff = cursor ^ target;
        let ones = diff.ones();

        for i in ones {
            let mut next = cursor;

            if reversed {
                // We need to drop our ones
                next[i] = false;
            } else {
                // We need to insert our ones
                next[i] = true;
            }

            pending.push_back(next);
        }

        Ok(Self {
            cursor,
            origin,
            target,
            seen,
            pending,
            reversed,
            initialized,
        })
    }

    fn inner_next(&mut self) {

    }
}

// TODO this should be breadth-first (but current implementation eats memory)
impl<const N: usize> Iterator for Converge<N> {
    type Item = Bits<N>;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("HASH {:?} VEC {:?}", self.seen.len(), self.pending.len());
        if !self.initialized {
            self.initialized = true;

            return Some(self.cursor);
        } else if self.origin == self.target {
            return None;
        }

        if !self.pending.is_empty() {
            // Safe to `unwrap` since we've checked for `is_empty`
            self.cursor = self.pending.pop_front().unwrap();

            loop {
                let diff = self.cursor ^ self.target;
                let ones = diff.ones();

                for i in ones {
                    let mut next = self.cursor;

                    if self.reversed {
                        // We need to drop our ones
                        next[i] = false;
                    } else {
                        // We need to insert our ones
                        next[i] = true;
                    }

                    if !self.seen.contains(&next) {
                        self.pending.push_front(next);
                    }
                }

                if self.seen.contains(&self.cursor) {
                    self.cursor = self.pending.pop_front().unwrap();

                    continue;
                }

                self.seen.insert(self.cursor);

                return Some(self.cursor);
            }
        }

        None
    }
}

type Path<const N: usize> = Vec<Bits<N>>;

#[derive(Debug)]
pub struct Paths<const N: usize> {
    origin: Bits<N>,
    target: Bits<N>,
    pending: VecDeque<(Bits<N>, Path<N>)>,
    reversed: bool,
    done: bool,
}

impl<const N: usize> Paths<N> {
    fn new(start: &Bits<N>, end: &Bits<N>) -> Result<Self, IncomparableError> {
        if start.partial_cmp(end).is_none() {
            return Err(IncomparableError);
        }

        let reversed: bool;
        let origin: Bits<N>;
        let target: Bits<N>;

        if start > end {
            reversed = true;
            origin = *end;
            target = *start;
        } else {
            reversed = false;
            origin = *start;
            target = *end;
        }

        let base = vec![origin];
        let diff = origin ^ target;
        let mut pending: VecDeque<(Bits<N>, Path<N>)> = VecDeque::new();

        for i in diff.ones() {
            let mut next = origin;
            next[i] = true;
            pending.push_back((next, base.clone()));
        }

        Ok(Self {
            origin,
            target,
            pending,
            reversed,
            done: false,
        })
    }
}

impl<const N: usize> Iterator for Paths<N> {
    type Item = Path<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.origin == self.target && !self.done {
            self.done = true;

            return Some(vec![self.origin]);
        } else if self.pending.is_empty() || self.done {
            return None;
        }

        let (cursor, mut base) = self.pending.pop_front().unwrap();
        base.push(cursor);

        if cursor == self.target {
            if self.reversed {
                base.reverse();
            }

            return Some(base);
        }

        let diff = cursor ^ self.target;

        for i in diff.ones() {
            let mut next = cursor;
            next[i] = true;
            self.pending.push_front((next, base.clone()));
        }

        self.next()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Bits<const N: usize> {
    inner: [bool; N],
}

impl<const N: usize> Bits<N> {
    pub fn new(value: bool) -> Self {
        Self { inner: [value; N] }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn count_zeroes(&self) -> usize {
        let mut count = 0_usize;

        for i in 0..self.len() {
            if !self[i] {
                count += 1;
            }
        }

        count
    }

    pub fn count_ones(&self) -> usize {
        let mut count = 0_usize;

        for i in 0..self.len() {
            if self[i] {
                count += 1;
            }
        }

        count
    }

    pub fn zeroes(&self) -> Zeroes<N> {
        Zeroes::new(self)
    }

    pub fn ones(&self) -> Ones<N> {
        Ones::new(self)
    }

    pub fn and(&self) -> bool {
        for i in 0..self.len() {
            if !self[i] {
                return false;
            }
        }

        true
    }

    pub fn or(&self) -> bool {
        for i in 0..self.len() {
            if self[i] {
                return true;
            }
        }

        false
    }

    pub fn distance(&self, other: &Bits<N>) -> Result<usize, IncomparableError> {
        if self.partial_cmp(other).is_none() {
            return Err(IncomparableError);
        }

        Ok((*self ^ *other).count_ones())
    }

    pub fn midpoints(&self, other: &Bits<N>) -> Result<Midpoints<N>, IncomparableError> {
        Midpoints::new(self, other)
    }

    pub fn converge(&self, other: &Bits<N>) -> Result<Converge<N>, IncomparableError> {
        Converge::new(self, other)
    }

    pub fn paths(&self, other: &Bits<N>) -> Result<Paths<N>, IncomparableError> {
        Paths::new(self, other)
    }

    pub fn lower_shadow(&self) -> LowerShadow<N> {
        LowerShadow::new(self)
    }

    pub fn upper_shadow(&self) -> UpperShadow<N> {
        UpperShadow::new(self)
    }
}

impl<const N: usize> PartialOrd for Bits<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        } else if self.le(other) {
            return Some(Ordering::Less);
        } else if other.le(self) {
            return Some(Ordering::Greater);
        }

        None
    }

    fn le(&self, other: &Self) -> bool {
        for i in 0..self.len() {
            if self[i] && !other[i] {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub enum ParseBitsError {
    LengthMismatch,
    NonBinary,
}

impl fmt::Display for ParseBitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseBitsError::LengthMismatch => {
                write!(f, "ParseBitsError::LengthMismatch")
            }
            ParseBitsError::NonBinary => {
                write!(f, "ParseBitsError::NonBinary")
            }
        }
    }
}

impl Error for ParseBitsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl<const N: usize> FromStr for Bits<N> {
    type Err = ParseBitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != N {
            return Err(ParseBitsError::LengthMismatch);
        }

        let mut inner: [bool; N] = [false; N];

        for (i, c) in s.chars().enumerate() {
            if c == '0' {
                inner[i] = false;
            } else if c == '1' {
                inner[i] = true;
            } else {
                return Err(ParseBitsError::NonBinary);
            }
        }

        Ok(Self { inner })
    }
}

impl<const N: usize> BitAnd for Bits<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut inner: [bool; N] = [false; N];

        for i in 0..self.len() {
            inner[i] = self[i] & rhs[i];
        }

        Self { inner }
    }
}

impl<const N: usize> BitAndAssign for Bits<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..self.len() {
            self.inner[i] = self[i] & rhs[i];
        }
    }
}

impl<const N: usize> BitOr for Bits<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut inner: [bool; N] = [false; N];

        for i in 0..self.len() {
            inner[i] = self[i] | rhs[i];
        }

        Self { inner }
    }
}

impl<const N: usize> BitOrAssign for Bits<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..self.len() {
            self.inner[i] = self[i] | rhs[i];
        }
    }
}

impl<const N: usize> BitXor for Bits<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut inner: [bool; N] = [false; N];

        for i in 0..self.len() {
            inner[i] = self[i] ^ rhs[i];
        }

        Self { inner }
    }
}

impl<const N: usize> BitXorAssign for Bits<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..self.len() {
            self.inner[i] = self[i] ^ rhs[i];
        }
    }
}

impl<const N: usize> Index<usize> for Bits<N> {
    type Output = bool;

    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

impl<const N: usize> IndexMut<usize> for Bits<N> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.inner[i]
    }
}

impl<const N: usize> Not for Bits<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut inner: [bool; N] = [false; N];

        for i in 0..self.len() {
            inner[i] = !self[i]
        }

        Self { inner }
    }
}

impl<const N: usize> Shl<usize> for Bits<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut inner: [bool; N] = [false; N];

        if rhs >= self.len() {
            Self { inner }
        } else {
            for i in 0..self.len() - rhs {
                inner[i] = self[rhs + i];
            }

            Self { inner }
        }
    }
}

impl<const N: usize> ShlAssign<usize> for Bits<N> {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

impl<const N: usize> Shr<usize> for Bits<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut inner: [bool; N] = [false; N];

        if rhs >= self.len() {
            Self { inner }
        } else {
            for i in 0..self.len() - rhs {
                inner[rhs + i] = self[i]
            }

            Self { inner }
        }
    }
}

impl<const N: usize> ShrAssign<usize> for Bits<N> {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl<const N: usize> fmt::Debug for Bits<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = "".to_string();

        for i in 0..self.len() {
            out.push_str(if self[i] { "1" } else { "0" });
        }

        write!(f, "{}", out)
    }
}

impl<const N: usize> fmt::Display for Bits<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out: String = "".to_string();

        for i in 0..self.len() {
            out.push_str(if self[i] { "1" } else { "0" });
        }

        write!(f, "{}", out)
    }
}
