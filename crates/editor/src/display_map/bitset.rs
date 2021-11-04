use smallvec::SmallVec;

pub struct BitSet {
    header: u64,
    bits: SmallVec<[u64; 2]>,
}

impl BitSet {
    fn new() -> Self {
        Self {
            header: 0,
            bits: SmallVec::new(),
        }
    }

    fn insert(&mut self, element: usize) {
        let container_ix = element / 64;
        let bits_ix = (self.header & ((1 << container_ix) - 1)).count_ones() as usize;
        if self.header & (1 << container_ix) == 0 {
            self.bits.insert(bits_ix, 1u64 << (element % 64));
        } else {
            self.bits[bits_ix] |= 1u64 << (element % 64);
        }
        self.header |= 1 << container_ix;
    }

    fn elements(&self) -> Vec<usize> {
        let mut elements = Vec::new();
        let mut bits_ix = 0;
        for ix in 0..64 {
            if self.header & (1 << ix) != 0 {
                let bits = self.bits[bits_ix];
                for offset in 0..64 {
                    if bits & (1 << offset) != 0 {
                        elements.push(ix * 64 + offset);
                    }
                }
                bits_ix += 1;
            }
        }
        elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        dbg!(std::mem::size_of::<BitSet>());
        let mut set = BitSet::new();
        set.insert(1);
        dbg!(set.elements());
        set.insert(10);
        dbg!(set.elements());
        set.insert(20);
        dbg!(set.elements());
        set.insert(100);
        set.insert(1000);
        set.insert(32);
        dbg!(set.elements());
    }
}
