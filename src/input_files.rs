use hashbrown::HashMap;
use std::str;
use std::{ffi::OsStr, fs, path::PathBuf};

pub fn get_input_files(name_dir: &str) -> HashMap<String, Vec<PathBuf>> {
    // get list of files from the input directory
    let l_files = list_files(name_dir).unwrap();

    // combine files into samples
    combine_files(l_files)
}

fn list_files(dir: &str) -> std::io::Result<Vec<PathBuf>> {
    print!(" . get files from input dir");

    let mut result = vec![];

    for path in fs::read_dir(dir)? {
        let path = path?.path();
        if let Some(ext) = path.extension().and_then(OsStr::to_str) {
            if ext == "gz" || ext == "bam" || ext == "BAM" {
                result.push(path.to_owned());
            }
        }
    }
    println!("	({} files)", result.len());
    Ok(result)
}

fn combine_files(vect_files: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    print!(" . combine files into samples");

    let mut results: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in vect_files {
        let filename = file.file_name().unwrap().to_str().unwrap();

        // check extension
        if filename.ends_with(".fastq.gz") || filename.ends_with(".fq.gz") {
            let mut sample = filename.replace(".fastq.gz", "").replace(".fq.gz", "");

            if sample.ends_with("_1") {
                sample = sample.trim_end_matches("_1").to_string();
            }
            if sample.ends_with("_2") {
                sample = sample.trim_end_matches("_2").to_string();
            }

            match results.get(&sample) {
                Some(_vect_files) => {
                    results.get_mut(&sample).unwrap().push(file);
                }
                None => {
                    results.insert(sample.to_owned(), Vec::new());
                    results.get_mut(&sample).unwrap().push(file);
                }
            }
        } else if filename.ends_with(".fas.gz")
            || filename.ends_with(".fasta.gz")
            || filename.ends_with(".fna.gz")
        {
            let sample = filename
                .replace(".fas.gz", "")
                .replace(".fasta.gz", "")
                .replace(".fna.gz", "");

            match results.get(&sample) {
                Some(_vect_files) => {
                    results.get_mut(&sample).unwrap().push(file);
                }
                None => {
                    results.insert(sample.to_owned(), Vec::new());
                    results.get_mut(&sample).unwrap().push(file);
                }
            }
        } else if filename.ends_with(".bam") || filename.ends_with(".BAM") {
            let sample = filename.replace(".bam", "").replace(".BAM", "");

            match results.get(&sample) {
                Some(_vect_files) => {
                    results.get_mut(&sample).unwrap().push(file);
                }
                None => {
                    results.insert(sample.to_owned(), Vec::new());
                    results.get_mut(&sample).unwrap().push(file);
                }
            }
        }
    }
    println!("	({} samples)", results.len());
    results
}
