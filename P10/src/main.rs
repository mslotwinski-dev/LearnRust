use rand::Rng;
use std::env;

const MUTATION_RATE: f64 = 0.01;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <password>", args[0]);
        return;
    }

    let true_password = &args[1];

    let mut population: Vec<String> = (0..10000)
        .map(|_| generate_password(true_password))
        .collect();

    let mut i = 0;
    loop {
        next_generation(&mut population, true_password);
        let best = &population[0];
        let score = compare(best.as_str(), true_password);
        println!("Best: {}, Score: {}", best, score);
        if score == true_password.len() {
            println!("Password cracked: {}, Generations: {}", best, i);
            break;
        }
        i += 1
    }
    // compare(password.as_str(), true_password);
}

fn next_generation(population: &mut Vec<String>, true_password: &str) {
    let mut rng = rand::rng();
    let mut new_population = Vec::new();

    population.sort_by_key(|p| std::cmp::Reverse(compare(p.as_str(), true_password)));

    let strongest: Vec<String> = population[..99].to_vec();

    while new_population.len() < population.len() {
        let parent1 = &strongest[rng.random_range(0..strongest.len())];
        let parent2 = &strongest[rng.random_range(0..strongest.len())];

        let crossover_point = rng.random_range(0..parent1.len());
        let mut child = String::new();
        child.push_str(&parent1[..crossover_point]);
        child.push_str(&parent2[crossover_point..]);

        child = mutate(child, true_password);
        new_population.push(child);
    }

    *population = new_population;
}

fn generate_password(true_password: &str) -> String {
    let mut rng = rand::rng();
    let password: String = (0..true_password.len())
        .map(|_| {
            let idx = rng.random_range(0..62);
            let c = if idx < 10 {
                (b'0' + idx as u8) as char
            } else if idx < 36 {
                (b'a' + (idx - 10) as u8) as char
            } else {
                (b'A' + (idx - 36) as u8) as char
            };
            c
        })
        .collect();
    password
}

fn compare(a: &str, b: &str) -> usize {
    if a.len() != b.len() {
        return 0;
    }

    a.chars().zip(b.chars()).filter(|(x, y)| x == y).count()
}

fn mutate(mut ind: String, true_password: &str) -> String {
    let mut rng = rand::rng();

    for i in 0..ind.len() {
        if rng.random::<f64>() < MUTATION_RATE {
            ind.replace_range(i..i + 1, &generate_password(true_password)[..1]);
        }
    }
    ind
}
