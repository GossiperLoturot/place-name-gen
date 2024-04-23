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

    let mut writter = csv::Writer::from_path("data/fst_term.csv").unwrap();
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

    let mut writter = csv::Writer::from_path("data/snd_term.csv").unwrap();
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

#[cfg(test)]
mod test {
    use rand::prelude::*;

    const COUNT: usize = 100;

    #[test]
    fn sample() {
        #[derive(Debug, serde::Deserialize)]
        struct InputRecord {
            term: String,
            local_freq: i64,
        }

        let mut fst_reader = csv::Reader::from_path("data/fst_term_l10n.csv").unwrap();
        let mut snd_reader = csv::Reader::from_path("data/snd_term_l10n.csv").unwrap();

        let mut fst_weight_col = vec![];
        let mut fst_term_col = vec![];
        for record in fst_reader.deserialize::<InputRecord>().flatten() {
            fst_weight_col.push(record.local_freq as f64);
            fst_term_col.push(record.term);
        }

        let mut snd_weight_col = vec![];
        let mut snd_term_col = vec![];
        for record in snd_reader.deserialize::<InputRecord>().flatten() {
            snd_weight_col.push(record.local_freq);
            snd_term_col.push(record.term);
        }

        let mut rng = thread_rng();

        let dist = rand::distributions::WeightedIndex::new(&fst_weight_col).unwrap();
        let fst_idx = dist.sample_iter(&mut rng).take(COUNT).collect::<Vec<_>>();

        let dist = rand::distributions::WeightedIndex::new(&snd_weight_col).unwrap();
        let snd_idx = dist.sample_iter(&mut rng).take(COUNT).collect::<Vec<_>>();

        for (fst, snd) in Iterator::zip(fst_idx.into_iter(), snd_idx.into_iter()) {
            println!("{}{}", fst_term_col[fst], snd_term_col[snd]);
        }
    }

    #[test]
    fn sample_l10n() {
        #[derive(Debug, serde::Deserialize)]
        struct InputRecord {
            local_freq: i64,
            en: String,
            jp: String,
        }

        let mut fst_reader = csv::Reader::from_path("data/fst_term_l10n.csv").unwrap();
        let mut snd_reader = csv::Reader::from_path("data/snd_term_l10n.csv").unwrap();

        let mut fst_weight_col = vec![];
        let mut fst_jp_col = vec![];
        let mut fst_en_col = vec![];
        for record in fst_reader.deserialize::<InputRecord>().flatten() {
            fst_weight_col.push(record.local_freq as f64);
            fst_en_col.push(record.en);
            fst_jp_col.push(record.jp);
        }

        let mut snd_weight_col = vec![];
        let mut snd_jp_col = vec![];
        let mut snd_en_col = vec![];
        for record in snd_reader.deserialize::<InputRecord>().flatten() {
            snd_weight_col.push(record.local_freq);
            snd_en_col.push(record.en);
            snd_jp_col.push(record.jp);
        }

        let mut rng = thread_rng();

        let dist = rand::distributions::WeightedIndex::new(&fst_weight_col).unwrap();
        let fst_idx = dist.sample_iter(&mut rng).take(COUNT).collect::<Vec<_>>();

        let dist = rand::distributions::WeightedIndex::new(&snd_weight_col).unwrap();
        let snd_idx = dist.sample_iter(&mut rng).take(COUNT).collect::<Vec<_>>();

        for (fst, snd) in Iterator::zip(fst_idx.into_iter(), snd_idx.into_iter()) {
            println!(
                "{}{} | {}{}",
                fst_en_col[fst], snd_en_col[snd], fst_jp_col[fst], snd_jp_col[snd],
            );
        }
    }
}