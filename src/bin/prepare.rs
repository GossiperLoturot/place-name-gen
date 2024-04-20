use symspell::*;

fn main() {
    let mut spell: SymSpell<AsciiStringStrategy> = SymSpell::default();
    spell.load_dictionary("frequency_dictionary_en_82_765.txt", 0, 1, " ");
    spell.load_bigram_dictionary("frequency_bigramdictionary_en_243_342.txt", 0, 2, " ");

    let mut reader = csv::ReaderBuilder::default()
        .has_headers(true)
        .delimiter(b'|')
        .from_path("city.csv")
        .unwrap();

    let mut cities = std::collections::HashSet::new();
    for record in reader.records() {
        let record = record.unwrap();
        let raw_city = record.get(0).unwrap();
        let city = raw_city.replace(" ", "").to_lowercase();
        cities.insert(city);
    }

    let mut rights = vec![];
    let mut lefts = vec![];
    for city in cities {
        let mut max_word_freq = 0;
        let mut bi_word = None;

        for i in 1..(city.len() - 1) {
            let (left, right) = city.split_at(i);
            let left_sug = spell.lookup(left, Verbosity::Top, 0);
            let right_sug = spell.lookup(right, Verbosity::Top, 0);

            if !left_sug.is_empty() && !right_sug.is_empty() {
                let word_freq = i64::min(left_sug[0].count, right_sug[0].count);

                if word_freq > max_word_freq {
                    bi_word = Some((left_sug[0].term.clone(), right_sug[0].term.clone()));
                    max_word_freq = word_freq;
                }
            }
        }

        if let Some((left, right)) = bi_word {
            lefts.push(left);
            rights.push(right);
        }
    }

    write_words_to_csv(&lefts, "left.csv");
    write_words_to_csv(&rights, "right.csv");
}

fn write_words_to_csv(words: &[String], file_name: &str) {
    let mut writter = csv::WriterBuilder::new().from_path(file_name).unwrap();

    let mut counter = std::collections::HashMap::<String, usize>::new();
    for word in words {
        *counter.entry(word.clone()).or_default() += 1;
    }

    let mut words = counter.into_iter().collect::<Vec<_>>();
    words.sort_by_key(|&(_, count)| count);

    for (word, count) in words.iter().rev() {
        writter.write_record(&[word, &count.to_string()]).unwrap();
    }
}
