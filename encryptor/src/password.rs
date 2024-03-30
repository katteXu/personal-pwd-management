use anyhow::{bail, Error, Ok};
use base64::engine::{general_purpose, Engine};
use hash::merhash::mersenne_hash;

const CRYPTO:&str = "!pqHr$*+ST1Vst_uv:?wWS%X&Y-/Z01_2.34<ABl9ECo|x#yDE^F{GHEI[]JK>LM#NOBWPQ:RaKU@}cde56R7=8f/9gIhi,jkzmn";

/// Generate a password from a seed and a length.
/// # 示例
/// ```
/// use encryptor::password::generate_password;
/// let pwd = generate_password("katte", 16);
/// match pwd {
///     Ok(p) => println!("{}", p),
///     Err(e) => println!("{}", e),
/// };
/// ```
pub fn generate_password(seed: &str, length: usize) -> Result<String, Error> {
    // Check if the length is valid
    if length < 6 {
        bail!("Password length must be at least 6 characters");
    }

    // Calculate the power of the Mersenne prime
    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };
    let mut mer_hash = mersenne_hash(seed).pow(p);

    // Generate the password by mer_hash
    let mut password = String::new();
    let crypto_len = CRYPTO.len();
    while mer_hash > 9 {
        let ioc = mer_hash % crypto_len;
        let nthc = CRYPTO.chars().nth(ioc).expect("Error while getting char!");
        password.push(nthc);
        mer_hash /= crypto_len;
    }

    // Add the seed to the password
    let interval = password.clone();
    for c in seed.chars() {
        password.push(c);
        password += &interval;
    }

    password = general_purpose::STANDARD.encode(password);

    password = password.replace("/", "*").replace("+", "*");

    // Truncate the password to the desired length
    let interval = password.clone();
    while password.len() < length {
        password += &interval;
    }

    Ok(format!("{}{}", seed, &password[..length]))
}
