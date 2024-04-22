import csv
import alkana

def l10n(input_file, output_file):
    input_file = open(input_file)
    output_file = open(output_file, "w")

    rdr = csv.DictReader(input_file)
    wtr = csv.DictWriter(output_file, ["term", "local_freq", "global_freq", "en", "jp"])

    wtr.writeheader()

    for row in rdr:
        en = row["term"]
        jp = alkana.get_kana(row["term"])

        if jp is None:
            continue

        wtr.writerow({ "en": en, "jp": jp, **row })

    input_file.close()
    output_file.close()

l10n("fst_term.csv", "fst_term_l10n.csv")
l10n("snd_term.csv", "snd_term_l10n.csv")
