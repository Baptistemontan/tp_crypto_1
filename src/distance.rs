use std::ops::BitXor;

pub trait Distance: Default {
    fn distance(a: &[u8], b: &[u8]) -> usize;
}

#[derive(Debug, Default)]
pub struct BitDiff {}

impl Distance for BitDiff {
    fn distance(a: &[u8], b: &[u8]) -> usize {
        fn bit_diff((a, b): (u8, u8)) -> usize {
            a.bitxor(b).count_ones() as usize
        }

        let a = a.into_iter().copied();
        let b = b.into_iter().copied();
        std::iter::zip(a, b).map(bit_diff).sum()
    }
}

#[derive(Debug, Default)]
pub struct ByteDiff {}

impl Distance for ByteDiff {
    fn distance(a: &[u8], b: &[u8]) -> usize {
        a.into_iter()
            .zip(b.into_iter())
            .filter(|(a, b)| a != b)
            .count()
    }
}

#[derive(Debug, Default)]
pub struct CommonSubSeq {}

impl Distance for CommonSubSeq {
    fn distance(a: &[u8], b: &[u8]) -> usize {
        let mut max = 0;

        let len = a.len();

        for i in 0..len {
            for j in 0..len {
                let size = a[i..]
                    .iter()
                    .zip(&b[j..])
                    .take_while(|(a, b)| a == b)
                    .count();
                if max < size {
                    max = size;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod bit_diff {
    use super::*;

    #[test]
    fn bit_diff_equal() {
        let bits_1 = hex::decode("ffffffff").unwrap();
        let bits_2 = hex::decode("ffffffff").unwrap();
        let bit_diff = BitDiff::distance(&bits_1, &bits_2);
        assert_eq!(bit_diff, 0);
    }

    #[test]
    fn bit_diff_1() {
        // 11111111101001010000101101110100
        let bits_1 = hex::decode("ffa50b74").unwrap();
        // 11111110101001100000101010000100
        let bits_2 = hex::decode("fea60a84").unwrap();
        // xor:
        // 00000001000000110000000111110000
        // popcnt = 8
        let bit_diff = BitDiff::distance(&bits_1, &bits_2);
        assert_eq!(bit_diff, 8);
    }

    #[test]
    fn bit_diff_different() {
        let bits_1 = hex::decode("00000000").unwrap();
        let bits_2 = hex::decode("ffffffff").unwrap();
        let bit_diff = BitDiff::distance(&bits_1, &bits_2);
        assert_eq!(bit_diff, bits_1.len() * 8);
    }
}

#[cfg(test)]
mod byte_diff {
    use super::*;

    #[test]
    fn byte_diff_equal() {
        let bytes_1 = hex::decode("ffffffff").unwrap();
        let bytes_2 = hex::decode("ffffffff").unwrap();
        let byte_diff = ByteDiff::distance(&bytes_1, &bytes_2);
        assert_eq!(byte_diff, 0);
    }

    #[test]
    fn byte_diff_1() {
        let bytes_1 = hex::decode("ffa50b748a").unwrap();
        let bytes_2 = hex::decode("fea50a648a").unwrap();
        let byte_diff = ByteDiff::distance(&bytes_1, &bytes_2);
        // 2 common byte
        assert_eq!(byte_diff, bytes_1.len() - 2);
    }

    #[test]
    fn byte_diff_different() {
        let bytes_1 = hex::decode("00000000").unwrap();
        let bytes_2 = hex::decode("ffffffff").unwrap();
        let byte_diff = ByteDiff::distance(&bytes_1, &bytes_2);
        assert_eq!(byte_diff, bytes_1.len());
    }
}