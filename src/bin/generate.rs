use rand::prelude::*;

const COUNT: usize = 100;

fn main() {
    let mut writter = csv::WriterBuilder::new()
        .delimiter(b',')
        .from_path("generated.csv")
        .unwrap();

    let mut rng = thread_rng();
    let lefts = choose_multiple("left.csv", COUNT, &mut rng);
    let rights = choose_multiple("right.csv", COUNT, &mut rng);

    for (i, (left, right)) in Iterator::zip(lefts.into_iter(), rights.into_iter()).enumerate() {
        let index = format!("{}", i + 1);
        let word = format!("{}{}", left, right);
        writter.write_record(&[index, word]).unwrap();
    }
}

fn choose_multiple<R: Rng + ?Sized>(file_name: &str, amount: usize, rng: &mut R) -> Vec<String> {
    let mut reader = csv::ReaderBuilder::default()
        .delimiter(b',')
        .from_path(file_name)
        .unwrap();

    let mut words = vec![];
    let mut weights = std::collections::HashMap::new();
    for record in reader.records() {
        let record = record.unwrap();

        let word = record.get(0).unwrap().parse::<String>().unwrap();
        let weight = record.get(1).unwrap().parse::<f64>().unwrap();

        words.push(word.clone());
        weights.insert(word, weight);
    }

    words
        .choose_multiple_weighted(rng, amount, |x| *weights.get(x).unwrap())
        .unwrap()
        .cloned()
        .collect::<Vec<_>>()
}
