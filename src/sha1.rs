use sha1::Digest;

pub fn encode(input: String) -> Box<[u8]> {
    let mut hasher = sha1::Sha1::new();
    hasher.update(input);
    let r = &hasher.finalize() as &[u8];

    return Box::from(r);
}

