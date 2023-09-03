use std::num::NonZeroU8;

use clap::Parser;

mod wordlist;
use wordlist::WORDS;

/// Generate a random passphrase
#[derive(Parser)]
struct Args {
    /// Number of words
    #[arg(default_value_t = NonZeroU8::new(7).unwrap())]
    num_words: NonZeroU8,
}

fn main() {
    use rand::rngs::OsRng;

    const WORDS_LEN: usize = 7776;
    assert_eq!(WORDS.len(), WORDS_LEN);
    const MASK: u16 = WORDS_LEN.next_power_of_two() as u16 - 1;

    let Args { num_words } = Args::parse();
    let num_words = num_words.get();

    let mut rng = OsRng;

    let mut rand_word = || loop {
        use rand::Rng;

        let i = usize::from(rng.gen::<u16>() & MASK);

        if let Some(word) = WORDS.get(i) {
            break *word;
        }
    };

    let phrase = (0..num_words)
        .map(|_| rand_word())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{phrase}");
}
