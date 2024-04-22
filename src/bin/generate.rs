use rand::prelude::*;

const COUNT: usize = 1000;

#[derive(Debug, serde::Deserialize)]
struct InputRecord {
    local_freq: i64,
    en: String,
    jp: String,
}

#[derive(Debug, serde::Serialize)]
struct OutputRecord {
    en: String,
    jp: String,
}

fn main() {
    let mut fst_reader = csv::Reader::from_path("fst_term_l10n.csv").unwrap();
    let mut snd_reader = csv::Reader::from_path("snd_term_l10n.csv").unwrap();

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

    let mut writer = csv::Writer::from_path("generated.csv").unwrap();
    for (fst, snd) in Iterator::zip(fst_idx.into_iter(), snd_idx.into_iter()) {
        writer
            .serialize(OutputRecord {
                en: format!("{}{}", fst_en_col[fst], snd_en_col[snd]),
                jp: format!("{}{}", fst_jp_col[fst], snd_jp_col[snd]),
            })
            .unwrap();
    }
}
