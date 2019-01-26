//! Memorization. Not quite memoization because the memory must be passed in.
use std::collections::HashMap;
use std::hash::Hash;

pub fn recall<T, R, F>(
    request: T, memory: &mut HashMap<T, R>, calculate: F
) -> R
where T: Clone + PartialEq + Eq + Hash,
      R: Clone,
      F: Fn(T) -> R,
{
    if let Some(r) = memory.get(&request) {
        return r.clone();
    }

    let r = (calculate)(request.clone());
    memory.insert(request, r.clone());

    r
}

#[cfg(test)]
mod test {
    use super::*;

    fn calculate(param1: u64, param2: u32) -> u128 {
        let param1 = param1 as u128;
        let param2 = param2 as u128;
        param1 * param1 - param2
    }

    #[test]
    fn memorizes() {
        let mut memory = HashMap::new();

        let r1p1 = 19;
        let r1p2 = 2;
        let o1 = calculate(r1p1, r1p2);

        let r2p1 = 100;
        let r2p2 = 101;
        let o2 = calculate(r2p1, r2p2);

        let r3p1 = 77;
        let r3p2 = 3;
        let o3 = calculate(r3p1, r3p2);

        assert!(memory.len() == 0);
        assert!(o1 == recall((r1p1, r1p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 1);
        assert!(o1 == recall((r1p1, r1p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 1);
        assert!(o1 == recall((r1p1, r1p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 1);
        assert!(o2 == recall((r2p1, r2p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 2);
        assert!(o1 == recall((r1p1, r1p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 2);
        assert!(o3 == recall((r3p1, r3p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 3);
        assert!(o2 == recall((r2p1, r2p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 3);
        assert!(o1 == recall((r1p1, r1p2), &mut memory, |(p1, p2)| calculate(p1, p2)));
        assert!(memory.len() == 3);
    }
}
