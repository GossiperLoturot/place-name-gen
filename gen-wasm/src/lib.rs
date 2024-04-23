use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod data;

#[wasm_bindgen(getter_with_clone)]
pub struct OutputRecord {
    pub en: String,
    pub jp: String,
}

#[wasm_bindgen]
pub fn sample_once() -> OutputRecord {
    let mut rng = thread_rng();

    let dist = rand::distributions::WeightedIndex::new(data::FST_WEIGHT).unwrap();
    let fst_idx = dist.sample(&mut rng);

    let dist = rand::distributions::WeightedIndex::new(data::SND_WEIGHT).unwrap();
    let snd_idx = dist.sample(&mut rng);

    OutputRecord {
        en: format!("{}{}", data::FST_EN[fst_idx], data::SND_EN[snd_idx]),
        jp: format!("{}{}", data::FST_JP[fst_idx], data::SND_JP[snd_idx]),
    }
}

#[wasm_bindgen]
pub fn sample_many(n: usize) -> Vec<OutputRecord> {
    let mut rng = thread_rng();

    let dist = rand::distributions::WeightedIndex::new(data::FST_WEIGHT).unwrap();
    let fst_idx = dist.sample_iter(&mut rng).take(n).collect::<Vec<_>>();

    let dist = rand::distributions::WeightedIndex::new(data::SND_WEIGHT).unwrap();
    let snd_idx = dist.sample_iter(&mut rng).take(n).collect::<Vec<_>>();

    Iterator::zip(fst_idx.into_iter(), snd_idx.into_iter())
        .map(|(fst_idx, snd_idx)| OutputRecord {
            en: format!("{}{}", data::FST_EN[fst_idx], data::SND_EN[snd_idx]),
            jp: format!("{}{}", data::FST_JP[fst_idx], data::SND_JP[snd_idx]),
        })
        .collect()
}
