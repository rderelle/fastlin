use hashbrown::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn get_barcodes(file_name: PathBuf, kmer_size: &u8) -> (HashMap<Vec<u8>, String>, i64) {
    print!(" . get barcodes and genome size");

    // convert kmer_size to usize and calculate half kmer size
    let k = *kmer_size as usize;
    let half_k_size: usize = (k - 1) / 2;

    // initialise Hashmap and genome size
    let mut barcodes_id: HashMap<Vec<u8>, String> = HashMap::default();
    let mut genome_size = 0;

    // read barcode file
    let mut counter = 0;
    let csv = read_to_string(file_name).unwrap();
    for l in csv.lines() {
        let inserts = l.split("\t");
        let collection = inserts.collect::<Vec<&str>>();

        if collection[0] == "genome_size" {
            // convert str to integer
            let parsed_result = collection[1].parse::<i64>();
            // check if the conversion was successful
            match parsed_result {
                Ok(parsed_number) => genome_size = parsed_number,
                Err(_) => {
                    panic!("Failed to read the genome size in barcode file")
                }
            }
        } else {
            // build id
            let id = format!("{}__{}", &collection[0], counter);
            // extract both sides
            let left_side = &collection[1].as_bytes()[50 - half_k_size..];
            let middle = collection[2].as_bytes();
            let right_side = &collection[3].as_bytes()[..half_k_size];

            // build barcode
            let mut barcode: Vec<u8> = Vec::with_capacity(k);
            barcode.extend_from_slice(left_side);
            barcode.extend_from_slice(middle);
            barcode.extend_from_slice(right_side);

            // save barcode and reverse complement
            barcodes_id.insert(barcode.clone(), id.clone());

            let rev_comp = revcomp_u8(&barcode);
            barcodes_id.insert(rev_comp, id);

            counter += 1;
        }
    }
    // double-check we have the genome size
    if genome_size == 0 {
        panic!("The genome size is missing from the barcode file")
    }

    //println!("	({} barcodes and genome size {})", counter, genome_size);
    println!("	({} barcodes)", counter);

    (barcodes_id, genome_size)
}

fn revcomp_u8(seq: &[u8]) -> Vec<u8> {
    seq.iter()
        .rev()
        .map(|&b| match b {
            b'a' => b'T',
            b'c' => b'G',
            b'g' => b'C',
            b't' => b'A',
            b'A' => b'T',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            _ => b'N',
        })
        .collect()
}
