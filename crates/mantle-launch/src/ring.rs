use std::collections::VecDeque;

/// A byte buffer of bounded size that keeps the newest bytes and drops the oldest.
#[derive(Debug)]
pub struct Ring {
    bytes: VecDeque<u8>,
    capacity: usize,
}

impl Ring {
    pub fn new(capacity: usize) -> Self {
        Self {
            bytes: VecDeque::new(),
            capacity,
        }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    /// Appends `data`, dropping the oldest bytes beyond capacity. Returns how many bytes were
    /// dropped, counting any of `data` itself that did not fit.
    pub fn push(&mut self, data: &[u8]) -> usize {
        let overflow = (self.bytes.len() + data.len()).saturating_sub(self.capacity);
        if data.len() >= self.capacity {
            self.bytes.clear();
            self.bytes.extend(&data[data.len() - self.capacity..]);
            return overflow;
        }
        self.bytes.drain(..overflow);
        self.bytes.extend(data);
        overflow
    }

    /// Appends the contents of another ring, oldest first.
    pub fn push_ring(&mut self, other: &Ring) {
        let (head, tail) = other.bytes.as_slices();
        self.push(head);
        self.push(tail);
    }

    /// The oldest contiguous run of bytes; empty only when the ring is empty.
    pub fn front(&self) -> &[u8] {
        let (head, tail) = self.bytes.as_slices();
        if head.is_empty() { tail } else { head }
    }

    /// Drops the `count` oldest bytes.
    pub fn consume(&mut self, count: usize) {
        let count = count.min(self.bytes.len());
        self.bytes.drain(..count);
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_everything_within_capacity() {
        let mut ring = Ring::new(8);
        ring.push(b"abc");
        ring.push(b"def");
        assert_eq!(ring.to_vec(), b"abcdef");
        assert_eq!(ring.len(), 6);
    }

    #[test]
    fn drops_the_oldest_bytes_on_overflow() {
        let mut ring = Ring::new(8);
        ring.push(b"abcdef");
        ring.push(b"ghij");
        assert_eq!(ring.to_vec(), b"cdefghij");
    }

    #[test]
    fn a_push_larger_than_capacity_keeps_its_tail() {
        let mut ring = Ring::new(4);
        ring.push(b"xy");
        ring.push(b"0123456789");
        assert_eq!(ring.to_vec(), b"6789");
        ring.push(b"abcd");
        assert_eq!(ring.to_vec(), b"abcd");
    }

    #[test]
    fn front_and_consume_drain_in_order_across_wraparound() {
        let mut ring = Ring::new(6);
        ring.push(b"abcdef");
        ring.consume(4);
        ring.push(b"ghij");
        let mut drained = Vec::new();
        while !ring.is_empty() {
            let chunk = ring.front().to_vec();
            assert!(!chunk.is_empty());
            drained.extend_from_slice(&chunk[..1]);
            ring.consume(1);
        }
        assert_eq!(drained, b"efghij");
        ring.consume(10);
        assert!(ring.front().is_empty());
    }

    #[test]
    fn push_ring_copies_oldest_first_and_respects_the_target_capacity() {
        let mut source = Ring::new(6);
        source.push(b"abcdef");
        source.consume(2);
        source.push(b"gh");
        let mut copy = Ring::new(16);
        copy.push_ring(&source);
        assert_eq!(copy.to_vec(), b"cdefgh");
        let mut small = Ring::new(3);
        small.push_ring(&source);
        assert_eq!(small.to_vec(), b"fgh");
    }

    #[test]
    fn push_reports_how_many_bytes_it_dropped() {
        let mut ring = Ring::new(8);
        assert_eq!(ring.push(b"abcdef"), 0);
        assert_eq!(ring.push(b"gh"), 0);
        assert_eq!(ring.push(b"ij"), 2);
        assert_eq!(ring.push(b"0123456789"), 10);
        assert_eq!(ring.to_vec(), b"23456789");
    }

    #[test]
    fn clear_empties() {
        let mut ring = Ring::new(4);
        ring.push(b"ab");
        ring.clear();
        assert!(ring.is_empty());
    }
}
