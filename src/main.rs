use chrono::prelude::*;
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::cast::ToPrimitive;

fn lcg(seed: &BigInt, a: &BigInt, c: &BigInt, m: &BigInt) -> BigInt {
    (seed * a + c) % m
}

fn generate_domain(seed: &BigInt) -> String {
    let a = 1664525.to_bigint().unwrap();
    let c = 1013904223.to_bigint().unwrap();
    let m = (2 as u64).pow(32).to_bigint().unwrap();

    let domain_extensions = vec![".com", ".io", ".ca", ".us"];

    let mut rng_seed = seed.clone();

    // Randomize domain length between 5 and 15 characters
    rng_seed = lcg(&rng_seed, &a, &c, &m);
    let domain_length = (rng_seed.clone() % (15 - 5 + 1).to_bigint().unwrap()).to_usize().unwrap() + 5;

    let mut domain = String::new();
    for _ in 0..domain_length {
        rng_seed = lcg(&rng_seed, &a, &c, &m);
        let char_code = (rng_seed.clone() % 26.to_bigint().unwrap()).to_u8().unwrap();
        domain.push(('a' as u8 + char_code) as char);
    }

    // Choose a random domain extension
    rng_seed = lcg(&rng_seed, &a, &c, &m);
    let extension_index = (rng_seed % domain_extensions.len().to_bigint().unwrap()).to_usize().unwrap();
    domain.push_str(domain_extensions[extension_index]);

    domain
}

fn generate_domains_based_on_time(number_of_domains: usize) -> Vec<String> {
    let now = Utc::now().format("%Y%m%d").to_string();
    let seed_base = now.parse::<u32>().unwrap().to_bigint().unwrap();

    let mut domains = Vec::new();
    for i in 0..number_of_domains {
        let seed = &seed_base + i.to_bigint().unwrap();
        let domain = generate_domain(&seed);
        domains.push(domain);
    }

    domains
}

fn main() {
    let generated_domains = generate_domains_based_on_time(5);
    println!("{:?}", generated_domains);
}
