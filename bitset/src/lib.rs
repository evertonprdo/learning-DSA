pub struct BitSet {
    words: Vec<usize>,
    bit_shift: usize,
    bit_len: usize,
    size: usize,
}
impl BitSet {
    pub fn new(size: usize) -> Self {
        let bytes = std::mem::size_of::<usize>();

        let bit_len = bytes * 8;
        let bit_shift = bit_len.trailing_zeros() as usize;

        let n_words = (size + bit_len - 1) >> bit_shift;

        BitSet {
            words: vec![0; n_words],
            bit_shift,
            bit_len,
            size,
        }
    }

    fn get_word_and_bit(&self, idx: &usize) -> (usize, usize) {
        assert!(self.size > *idx);

        let word = idx >> self.bit_shift;
        let bit = idx % self.bit_len;

        (word, bit)
    }

    pub fn set(&mut self, idx: usize) {
        let (word, bit) = self.get_word_and_bit(&idx);

        self.words[word] |= 1 << bit;
    }

    pub fn clear(&mut self, idx: usize) {
        let (word, bit) = self.get_word_and_bit(&idx);

        self.words[word] &= 0 << bit;
    }

    pub fn toggle(&mut self, idx: usize) {
        let (word, bit) = self.get_word_and_bit(&idx);

        self.words[word] ^= 1 << bit;
    }

    pub fn is_set(&self, idx: usize) -> bool {
        let (word, bit) = self.get_word_and_bit(&idx);

        self.words[word] & (1 << bit) != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_func<F>(f: F, bitset: &mut BitSet, expect: bool, idx: usize)
    where
        F: Fn(&mut BitSet, usize),
    {
        assert_eq!(bitset.is_set(idx), !expect);

        f(bitset, idx);
        assert_eq!(bitset.is_set(idx), expect);
    }

    #[test]
    fn test_set() {
        let mut set = BitSet::new(256);

        for n in [0, 3, 63, 127, 255] {
            assert_func(BitSet::set, &mut set, true, n);
            assert_func(BitSet::clear, &mut set, false, n);
            assert_func(BitSet::toggle, &mut set, true, n);
            assert_func(BitSet::toggle, &mut set, false, n);
        }
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let set = BitSet::new(256); // 0 - 255
        set.is_set(256);
    }
}
