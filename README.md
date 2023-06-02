# Domain Generation Algorithm (DGA)

This Rust program generates random domain names based on a seed value using a linear congruential generator (LCG) algorithm. The generated domains are a combination of random characters and a randomly chosen domain extension.

## Algorithm

The program implements the LCG algorithm to generate pseudorandom numbers. The LCG function takes a seed value and parameters `a`, `c`, and `m` as inputs. It calculates the next pseudorandom number using the formula `(seed * a + c) % m`. The generated number becomes the new seed for the next iteration.

The `generate_domain` function generates a single random domain name based on a given seed. It first initializes the LCG parameters `a`, `c`, and `m`. Then, it generates a random domain length between 5 and 15 characters by applying the LCG algorithm. Next, it generates each character of the domain by applying the LCG algorithm and converting the generated number to a character code. Finally, it chooses a random domain extension from a predefined list and appends it to the domain name.

The `generate_domains_based_on_time` function generates a specified number of random domains based on the current date and time. It uses the current date in the format `YYYYMMDD` as the base seed value. It then iterates from 0 to the desired number of domains, incrementing the seed value by 1 for each iteration. It calls the `generate_domain` function with the incremented seed and collects the generated domains into a vector.

The `main` function demonstrates the usage of the above functions by generating 5 random domains based on the current time and printing them.

## Usage

To use this program, follow these steps:

1. Clone the repository:
```bash
git clone [https://github.com/your-username/random-domain-generator.git](https://github.com/SourM1lk/domain_generation_algorithm.git)
```
2. Navigate to the project directory:
```bash
cd random-domain-generator
```

3. Build the program using Cargo:
```bash
cargo build --release
```

4. Run the program

  
## License

[![Beerware License](https://img.shields.io/badge/License-Beerware-yellow.svg)](https://en.wikipedia.org/wiki/Beerware)

This project is licensed under the Beerware License. If you found this project enjoyable or useful, and we meet someday, you can buy me a beer in appreciation. Cheers! 🍻
