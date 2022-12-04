use clap::Parser;

fn wordlist() -> Vec<&'static str> {
    static WORDLIST: &str = include_str!("eff_large_wordlist.txt");

    WORDLIST.lines().map(str::trim).collect()
}

/// Generate a random passphrase
#[derive(Parser)]
struct Args {
    /// Number of words
    #[arg(default_value_t = 7)]
    num_words: u8,
}

fn main() {
    use rand::rngs::OsRng;

    let wordlist = wordlist();
    let args = Args::parse();

    let mut rng = OsRng;

    let mut rand_word = || loop {
        use rand::Rng;

        const MASK: u16 = (1 << 13) - 1;

        let i = usize::from(rng.gen::<u16>() & MASK);

        if let Some(word) = wordlist.get(i) {
            break *word;
        }
    };

    let phrase: String = (0..args.num_words)
        .map(|_| rand_word())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", phrase);
}
