## fastlin

# Overview

Fastlin is'\n

__Reference:__ [TBA](https://www)

# Installation
To install fastlin, you must have the [rust toolchain](https://www.rust-lang.org/tools/install) installed.
```
cargo install fastlin
```
Or alternatively you can copy the code from this repository and install it using this command:
```
cargo install --path .
```

# Running fastlin
The minimalist command line is:
```
fastlin -d /path/directory_fastq_files -b barcodes_file.txt
```
I you are are sure the data are not BAM-derived fastq files, then we would recommend to apply a maximum kmer coverage threshold to reduce runtimes: 
```
fastlin -d /path/directory_fastq_files -b barcode_file.txt -x 80
```

# Input files



# Output file



# TO DO LIST
+ add multi-threading
+ add possiblity to analyse FASTA files (genome assemblies)



See "LICENSE" for full terms and conditions of usage.
