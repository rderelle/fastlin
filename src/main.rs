use std::fs::File;
use std::io::Write;
use std::str;
use clap::Parser;
use std::{process,path::PathBuf};
use indicatif::{ProgressBar, ProgressStyle};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::Mutex;

mod get_barcodes;
use get_barcodes::get_barcodes::get_barcodes;

mod input_files;
use input_files::input_files::get_input_files;

mod analyse_sample_fastq;
use analyse_sample_fastq::analyse_sample_fastq::scan_reads;

mod analyse_sample_fasta;
use analyse_sample_fasta::analyse_sample_fasta::scan_fasta;

mod analyse_sample_bam;
use analyse_sample_bam::analyse_sample_bam::scan_bam;

mod process_barcodes;
use process_barcodes::process_barcodes::process_barcodes;


#[derive(Parser, Debug)]
#[command(author = None, version, about = None, long_about = None)]
struct Args {
    /// directory containing the data files
    #[arg(short, long)]
    dir: String,

    /// file containing the reference barcodes
    #[arg(short = 'b', long)]
    barcodes: String,

    /// output file [out_fastlin.txt]
    #[arg(short = 'o', long, default_value_t = String::from("out_fastlin.txt"))]
    output: String,

    /// kmer size
    #[arg(short, long, default_value_t = 25)]
    kmer_size: u8,

    /// minimum number of kmer occurences
    #[arg(short = 'c', long, default_value_t = 4)]
    min_count: i32,
    
    /// minimum number of barcodes
    #[arg(short = 'n', long, default_value_t = 3)]
    n_barcodes: usize,

    /// maximum kmer coverage
    #[arg(short = 'x', long)]
    max_cov: Option<i64>,

    /// number of threads
    #[arg(short = 't', long, default_value_t = 1)]
    nb_threads: usize,
}


#[allow(unused_assignments)]
fn get_data_type(name_sample: String, vec_files:Vec<PathBuf>) -> String {
    // depending on the number of files, returns 'single', 'paired' or exit with error message
    
    let mut count_fasta = 0;
    let mut count_fastq = 0;
    let mut count_bam = 0;

    for file_path in vec_files {
        if let Some(file_str) = file_path.to_str() {
            if file_str.ends_with(".fna.gz") || file_str.ends_with(".fas.gz") || file_str.ends_with(".fasta.gz") {
                count_fasta += 1;
            } else if file_str.ends_with(".fq.gz") || file_str.ends_with(".fastq.gz") {
                count_fastq += 1;
            } else if file_str.ends_with(".bam") || file_str.ends_with(".BAM") {
                count_bam += 1;
            }
        }
    }
        
    let mut result = "";
    if count_fasta == 1 && count_fastq == 0  && count_bam == 0 {result = "assembly";}
    else if count_fasta == 0 && count_bam == 0 && count_fastq == 1  {result = "single";}
    else if count_fasta == 0 && count_bam == 0 && count_fastq == 2  {result = "paired";}
    else if count_fasta == 0 && count_fastq == 0 && count_bam == 1 {result = "BAM";}
    else {
        eprintln!("error: sample {} has {} fasta, {} fastq and {} BAM files", name_sample, count_fasta, count_fastq, count_bam);
        process::abort();
    }
    result.to_string()
}


fn main() {
    println!("\n      fastlin     \n");
    
    // get command line arguments
    let args = Args::parse();
    
    // check chosen kmer size
    if args.kmer_size < 11 || args.kmer_size > 99 || args.kmer_size % 2 == 0 {
        // warning message
        eprintln!(" Error: the kmer size should be an odd number between 11 and 99.\n");
        // exit fastlin
        std::process::exit(0);
    }
    
    // get reference barcodes
    let (barcodes, genome_size) = get_barcodes((&args.barcodes).into(), &args.kmer_size);
    
    // calculate maximum number of kmers to extract (and limit_kmer = true if such limit exists)
    let mut max_kmers = 0;
    let mut limit_kmer = false;
    match args.max_cov {
        Some(value) => {
            max_kmers = genome_size * value;
            limit_kmer = true; 
        }
        None => {}
    }
        
    // get samples and input files
    let all_samples = get_input_files(&args.dir);

    // sort samples
    let mut sorted_samples: Vec<_> = all_samples.iter().collect();
    sorted_samples.sort_by_key(|k| k.0);
    
    // create output file
    let mut output_file = File::create(args.output).expect("\n   Warning: couldn't not create output file.\n");
    output_file.write("#sample	data_type	k_cov	mixture	lineages	log_barcodes	log_errors\n".as_bytes()).expect("write failed!");

    // initialise progress bar
    let pb = ProgressBar::new(sorted_samples.len().try_into().unwrap());
    let sty = ProgressStyle::with_template("   {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}",).unwrap().progress_chars("##-");
    pb.set_style(sty);

    // set the fixed number of threads for the global thread pool
    ThreadPoolBuilder::new().num_threads(args.nb_threads).build_global().unwrap();

    let output_file = Mutex::new(output_file); // Wrap the output file in a Mutex
    
    // process samples 1 by 1
    println!(" . analyse all samples");
    sorted_samples.par_iter().for_each(|(sample, list_files)| {

        // progress bar
        pb.inc(1);
        
        // get sequencing type ('single' or 'paired' reads)
        let data_type = get_data_type(sample.to_string(), list_files.to_vec());
        
        if data_type == "assembly" {
             // analyse genome
             let (barcode_found, error_message) = scan_fasta(list_files.to_vec(), barcodes.to_owned(), &args.kmer_size);
             
             // process barcodes
             let (lineages, mixture, string_occurences) = process_barcodes(barcode_found, 1, args.n_barcodes);
       
             // write sample info into output file
             let mut output_file = output_file.lock().unwrap(); // Lock the mutex to get a thread-safe reference to the file
             write!(output_file, "{}\t{}\t{}\t{}\t{}\t{}\t{}\n", sample, data_type, "1", mixture, lineages, string_occurences, error_message).expect("Failed to write to file");

        } else if data_type == "BAM" {
             // analyse BAM file
             let (barcode_found, coverage, error_message) = scan_bam(list_files.to_vec(), barcodes.to_owned(), &args.kmer_size, genome_size);
             
             // process barcodes
             let (lineages, mixture, string_occurences) = process_barcodes(barcode_found, args.min_count, args.n_barcodes);
       
             // write sample info into output file
             let mut output_file = output_file.lock().unwrap(); // Lock the mutex to get a thread-safe reference to the file
             write!(output_file, "{}\t{}\t{}\t{}\t{}\t{}\t{}\n", sample, data_type, coverage, mixture, lineages, string_occurences, error_message).expect("Failed to write to file");
        
        } else if data_type == "single" || data_type == "paired" {
             // analyse reads
             let (barcode_found, coverage, error_message) = scan_reads(list_files.to_vec(), barcodes.to_owned(), &args.kmer_size, limit_kmer, max_kmers, genome_size);
             
             // process barcodes
             let (lineages, mixture, string_occurences) = process_barcodes(barcode_found, args.min_count, args.n_barcodes);
             
             // write sample info into output file
             let mut output_file = output_file.lock().unwrap(); // Lock the mutex to get a thread-safe reference to the file
             write!(output_file, "{}\t{}\t{}\t{}\t{}\t{}\t{}\n", sample, data_type, coverage, mixture, lineages, string_occurences, error_message).expect("Failed to write to file");
        
        }
        
    });    
    
    println!("   done.");
}
