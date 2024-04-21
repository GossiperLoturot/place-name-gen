use symspell::*;

#[derive(Debug, serde::Deserialize)]
struct InputRecord {
    #[serde(rename = "City")]
    city: String,
}

#[derive(Debug, serde::Serialize)]
struct OutputRecord {
    term: String,
    local_freq: i64,
    global_freq: i64,
}

fn main() {
    let mut spell: SymSpell<AsciiStringStrategy> = SymSpell::default();
    spell.load_dictionary("data/frequency_dictionary_en_82_765.txt", 0, 1, " ");
    spell.load_bigram_dictionary("data/frequency_bigramdictionary_en_243_342.txt", 0, 2, " ");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .from_path("data/city.csv")
        .unwrap();

    let mut text_col = std::collections::HashSet::new();
    for record in reader.deserialize::<InputRecord>().flatten() {
        let text = record.city;
        let text = text.replace(" ", "").to_lowercase();
        text_col.insert(text);
    }

    let mut global_freq = std::collections::HashMap::<String, i64>::new();
    let mut fst_col = std::collections::HashMap::<String, i64>::new();
    let mut snd_col = std::collections::HashMap::<String, i64>::new();
    for text in text_col {
        let mut fst_freq = 0;
        let mut snd_freq = 0;

        let mut fst_term = String::new();
        let mut snd_term = String::new();

        for i in 1..text.len() {
            let (fst, snd) = text.split_at(i);
            let mut fst_sug = spell.lookup(fst, Verbosity::Top, 0);
            let mut snd_sug = spell.lookup(snd, Verbosity::Top, 0);

            if fst_sug.is_empty() || snd_sug.is_empty() {
                continue;
            }

            let fst = fst_sug.swap_remove(0);
            let snd = snd_sug.swap_remove(0);

            if i64::min(fst.count, snd.count) > i64::min(fst_freq, snd_freq) {
                fst_freq = fst.count;
                snd_freq = snd.count;

                fst_term = fst.term;
                snd_term = snd.term;
            }
        }

        if fst_term.is_empty() || snd_term.is_empty() {
            continue;
        }

        global_freq.insert(fst_term.clone(), fst_freq);
        global_freq.insert(snd_term.clone(), snd_freq);
        *fst_col.entry(fst_term).or_default() += 1;
        *snd_col.entry(snd_term).or_default() += 1;
    }

    let mut writter = csv::Writer::from_path("fst_term.csv").unwrap();
    for (term, local_freq) in fst_col {
        let global_freq = *global_freq.get(&term).unwrap();
        writter
            .serialize(OutputRecord {
                term,
                local_freq,
                global_freq,
            })
            .unwrap();
    }

    let mut writter = csv::Writer::from_path("snd_term.csv").unwrap();
    for (term, local_freq) in snd_col {
        let global_freq = *global_freq.get(&term).unwrap();
        writter
            .serialize(OutputRecord {
                term,
                local_freq,
                global_freq,
            })
            .unwrap();
    }
}
