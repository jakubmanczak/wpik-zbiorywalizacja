use rand08::{Rng, SeedableRng, rngs::StdRng};

// from: jakubmanczak/quote-engine.git
pub fn generate_short_token() -> String {
    let mut bytes = [0u8; 8];
    StdRng::from_entropy().fill(&mut bytes);
    base32::encode(base32::Alphabet::Crockford, &bytes)
}
