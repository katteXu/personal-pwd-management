pub mod merhash;

#[cfg(test)]
mod tests {
    use crate::merhash::mersenne_hash;

    #[test]
    fn mersenne_hash_works() {
        let seed = "katte";
        let hash = mersenne_hash(seed);

        assert_eq!(830583, hash);
    }
}
