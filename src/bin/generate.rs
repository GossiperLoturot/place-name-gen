use rand::prelude::*;

const COUNT: usize = 1000;

#[derive(Debug, serde::Deserialize)]
struct InputRecord {
    term: String,
    local_freq: i64,
}

#[derive(Debug, serde::Serialize)]
struct OutputRecord {
    city: String,
}

fn main() {
    let mut fst_reader = csv::Reader::from_path("fst_term.csv").unwrap();
    let mut snd_reader = csv::Reader::from_path("snd_term.csv").unwrap();

    let mut fst_col = std::collections::HashMap::<String, i64>::new();
    for record in fst_reader.deserialize::<InputRecord>().flatten() {
        fst_col.insert(record.term, record.local_freq);
    }

    let mut snd_col = std::collections::HashMap::<String, i64>::new();
    for record in snd_reader.deserialize::<InputRecord>().flatten() {
        snd_col.insert(record.term, record.local_freq);
    }

    let mut rng = thread_rng();
    let fst_sample = fst_col.keys().collect::<Vec<_>>();
    let fst_sample = fst_sample
        .choose_multiple_weighted(&mut rng, COUNT, |x| fst_col[*x] as f64)
        .unwrap();
    let snd_sample = snd_col.keys().collect::<Vec<_>>();
    let snd_sample = snd_sample
        .choose_multiple_weighted(&mut rng, COUNT, |x| snd_col[*x] as f64)
        .unwrap();

    let mut writer = csv::Writer::from_path("generated.csv").unwrap();
    for (fst, snd) in Iterator::zip(fst_sample, snd_sample) {
        let city = format!("{}{}", fst, snd);
        writer.serialize(OutputRecord { city }).unwrap();
    }
}
