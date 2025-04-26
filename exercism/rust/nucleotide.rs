use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let seq_res = find_invalid_sequence(dna);
    let nuc_res = find_invalid_nucleotide(nucleotide);

    match [seq_res, nuc_res] {
        [Some(n), _] => Err(n),
        [_, Some(n)] => Err(n),
        [None, None] => {
            let cnt = dna.matches(nucleotide).collect::<Vec<_>>().len();
            Ok(cnt)
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    counts
        .iter_mut()
        .for_each(|(k, v)| *v = count_chars(*k, dna));

    Ok(counts)
}

fn count_chars(letter: char, string: &str) -> usize {
    string.matches(letter).collect::<Vec<_>>().len()
}

fn find_invalid_nucleotide(nucleotide: char) -> Option<char> {
    let res = "ACGT".find(nucleotide);
    match res {
        None => Some(nucleotide),
        Some(_) => None,
    }
}
fn find_invalid_sequence(dna: &str) -> Option<char> {
    let res = dna.find(|c: char| !['A', 'C', 'G', 'T'].contains(&c));
    match res {
        None => None,
        Some(idx) => dna.chars().nth(idx),
    }
}
