use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_string(length: Option<usize>) -> String {
    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length.unwrap_or(8))
        .map(char::from)
        .collect::<String>();
    random_string
}
