use flate2::read::MultiGzDecoder;
use hashbrown::HashMap;
use seq_io::fasta::{Reader, Record};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn scan_fasta(
    mut vect_files: Vec<PathBuf>,
    barcodes: HashMap<Vec<u8>, String>,
    k_size: &u8,
) -> (HashMap<String, i32>, String) {
    // initialise kmer size
    let k = *k_size as usize;

    // sort vector of paths
    vect_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut result_barcodes: HashMap<String, i32> = HashMap::new();
    let mut error_message: String = String::new();

    'label_loop: for filename in vect_files {
        // set the reader
        let buf = get_reader(&filename);
        let mut reader = Reader::new(buf);

        // lookup records 1 by 1
        while let Some(record) = reader.next() {
            // unwrap record (contains name, sequence and quality)
            let record_ready = match record {
                Ok(record) => record,
                Err(err) => {
                    // re-initialise barcode results (-> no result)
                    result_barcodes = HashMap::new();
                    // save error message
                    error_message = format!("Error in file {:?}: {}", filename, err);
                    // stop reading file(s)
                    break 'label_loop;
                }
            };

            // get sequences and sequence length
            let seq = record_ready.seq();

            // only consider sequences long enough to have a kmer
            if seq.len() >= k {
                // extract kmers (slices from Vect seq)
                for n in 0..(seq.len() - k + 1) {
                    // get slice of Vect[u8]
                    let kmer = &seq[n..n + k];

                    // check if kmer is known -> add to count if yes or create new count if no
                    if let Some(id) = barcodes.get(kmer) {
                        *result_barcodes.entry(id.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    (result_barcodes, error_message)
}

fn get_reader(path: &PathBuf) -> Box<dyn BufRead + Send> {
    let mut filetype = "unzip";
    let filename_str = path.to_str().unwrap();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Error opening compressed file: {:?}.", error),
    };
    if filename_str.ends_with(".gz") {
        filetype = "zip";
    }
    let reader: Box<dyn BufRead + Send> = match filetype {
        "zip" => Box::new(BufReader::new(MultiGzDecoder::new(file))),
        _ => Box::new(BufReader::new(file)),
    };
    reader
}
