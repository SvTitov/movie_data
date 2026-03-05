use rand::distr::{Alphanumeric, SampleString};

pub fn generate_token() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 32)
}