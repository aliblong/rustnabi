use ring::rand::SecureRandom;

pub fn hash(pw: &[u8]) -> Vec<u8> {
    use ring::digest;
    digest::digest(&digest::SHA256, pw).as_ref().to_vec()
}
