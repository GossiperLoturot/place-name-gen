use std::io::Write;

#[derive(Debug, serde::Deserialize)]
struct InputRecord {
    local_freq: i64,
    en: String,
    jp: String,
}

fn main() {
    let mut writter = std::fs::File::create("src/data.rs").unwrap();

    let mut reader = csv::Reader::from_path("data/fst_term_l10n.csv").unwrap();
    let mut weight = String::new();
    let mut en = String::new();
    let mut jp = String::new();
    for record in reader.deserialize::<InputRecord>().flatten() {
        weight.push_str(&format!("{}.0,", record.local_freq));
        en.push_str(&format!("\"{}\",", record.en));
        jp.push_str(&format!("\"{}\",", record.jp));
    }

    writeln!(writter, "pub const FST_WEIGHT: &[f64] = &[{}];", weight).unwrap();
    writeln!(writter, "pub const FST_EN: &[&str] = &[{}];", en).unwrap();
    writeln!(writter, "pub const FST_JP: &[&str] = &[{}];", jp).unwrap();

    let mut reader = csv::Reader::from_path("data/snd_term_l10n.csv").unwrap();
    let mut weight = String::new();
    let mut en = String::new();
    let mut jp = String::new();
    for record in reader.deserialize::<InputRecord>().flatten() {
        weight.push_str(&format!("{}.0,", record.local_freq));
        en.push_str(&format!("\"{}\",", record.en));
        jp.push_str(&format!("\"{}\",", record.jp));
    }

    writeln!(writter, "pub const SND_WEIGHT: &[f64] = &[{}];", weight).unwrap();
    writeln!(writter, "pub const SND_EN: &[&str] = &[{}];", en).unwrap();
    writeln!(writter, "pub const SND_JP: &[&str] = &[{}];", jp).unwrap();
}
