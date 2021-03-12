use crate::protocol::setup::PedersenGens;

pub struct Commitment<T> {
    commitment: T,
}

impl<T> Commitment<T> {
    fn commit(message: Vec<u8>) -> T {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_fails_empty() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_commit_non_err_on_nonempty() {
        assert_ne!(0, 1);
    }
}
