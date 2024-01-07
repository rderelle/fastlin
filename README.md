[![Anaconda-Server Badge](https://img.shields.io/badge/install%20with-bioconda-brightgreen.svg?style=flat)](https://bioconda.github.io/recipes/fastlin/README.html) 
[![Crates.io](https://img.shields.io/crates/v/fastlin)](https://crates.io/crates/fastlin)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/rderelle/fastlin)](https://github.com/rderelle/fastlin/releases)

<h1 align="center">fastlin</h1>


### Overview

Fastlin is an ultra-fast program to perform lineage typing of <i>Mycobacterium tuberculosis</i> complex (MTBC) FASTQ read data and FASTA assemblies. Using the split-kmer approach, it can accuratly predict MTBC lineages and strain mixtures in seconds.

Reference:[fastlin: an ultra-fast program for Mycobacterium tuberculosis complex lineage typing.](https://doi.org/10.1093/bioinformatics/btad648)


### Installation
To install fastlin via cargo, you must have the [rust toolchain](https://www.rust-lang.org/tools/install) installed.
```
cargo install fastlin
```
Or you can copy the code from this repository and install it using this command:
```
cargo install --path .
```
Alternatively, you can install precompiled binaries using Conda (Linux and macOS Intel processors):
```
conda install -c bioconda fastlin
```
You will also need a barcode file (see Input files below).

### Running fastlin
The default command line is:
```
fastlin -d /path/directory_fastq_files -b barcodes_file.txt
```
If your dataset does not contain any BAM-derived fastq file, then we would recommend to apply a maximum kmer coverage threshold to reduce runtimes: 
```
fastlin -d /path/directory_fastq_files -b barcode_file.txt -x 80
```

### Input files
<p>Fastlin takes as input the path of the directory containing the fastq and/or fasta files. The directory can contain a mix of FASTA geome assemblies, paired-end and single-end FASTQ files. These data files should be gzipped, with the following extensions:</p>

- **.fastq.gz** or **.fq.gz** for FASTQ read data. The names of paired-end files should be in the form name_1.fq.gz and name_2.fq.gz (or equivalent with fastq.gz)
- **.fas.gz**, **.fasta.gz** or **.fna.gz** for FASTA genome assemblies. In the cases of FASTA files, (i) the min-occurence paramter is automatically set to 1 and (ii) the maximum kmer coverage is ignored.

<p>The MTBC barcode file can be downloaded from https://www.github.com/rderelle/barcodes-fastlin. 
Alternatively, you can build and test your own kmer barcodes using the Python scripts available in that directory.</p> 

### Manual

A full description of fastlin parameters can be found [here](https://github.com/rderelle/fastlin/blob/main/parameters.md).

### Output file
Fastlin output consists of a tab-delimited file with the following fields:
+ sample: sample name
+ data type: 'assembly', 'single' (reads) or 'paired' (-end reads)
+ k_cov: theoretical kmer coverage of the fastq files(s) based on the number of extracted kmers
+ mixture: pure ('no') or mixed ('yes') sample
+ lineages: detected lineages (median kmer occurences within paratheses)
+ log_barcodes: kmer barcodes passing the minimum occurence threshold, indicated by their kmer occurence and grouped by lineages

Here is a simple example:
> #sample&nbsp;&nbsp;&nbsp;&nbsp;data type&nbsp;&nbsp;&nbsp;&nbsp;k_cov&nbsp;&nbsp;&nbsp;&nbsp;mixture&nbsp;&nbsp;&nbsp;&nbsp;lineages&nbsp;&nbsp;&nbsp;&nbsp;log_barcodes&nbsp;&nbsp;&nbsp;&nbsp;log_errors  
ERRxxxxx&nbsp;&nbsp;&nbsp;&nbsp;paired&nbsp;&nbsp;&nbsp;&nbsp;118&nbsp;&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;2 (45)&nbsp;&nbsp;&nbsp;&nbsp;2 (42, 48, 39, 43, 54, 47, 45), 4.1 (4)

The sample ERRxxxxx contains a single strain belonging to lineage 2. This typing is supported by 7 kmer barcodes, with a median number of occurences of 45. Since the abundance of the strain is far below the theoretical kmer coverage (equal here to 118), we can conclude that the sample is likely to contain high level of contaminations or sequencing errors.

### Error handling
<p>When fastlin cannot read a fastq file (e.g., faulty record within the fastq file, corrupt gzip file), it stops scanning it, re-initialise all values to 0 and report the error message in the last column of the output file. Here is an example of output with 3 different errors:</p>

> #sample&nbsp;&nbsp;&nbsp;&nbsp;nb_files&nbsp;&nbsp;&nbsp;&nbsp;k_cov&nbsp;&nbsp;&nbsp;&nbsp;mixture&nbsp;&nbsp;&nbsp;&nbsp;lineages&nbsp;&nbsp;&nbsp;&nbsp;log_barcodes&nbsp;&nbsp;&nbsp;&nbsp;log_errors  
dummy1&nbsp;&nbsp;&nbsp;single&nbsp;&nbsp;&nbsp;0&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Error in file "reads/dummy1.fastq.gz": FASTQ parse error: sequence length is 150, but quality length is 50 (record 'ERR551806.5' at line 17).  
dummy2&nbsp;&nbsp;&nbsp;single&nbsp;&nbsp;&nbsp;0&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Error in file "reads/dummy2.fastq.gz": invalid gzip header  
dummy3&nbsp;&nbsp;&nbsp;single&nbsp;&nbsp;&nbsp;0&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Error in file "reads/dummy3.fastq.gz": corrupt deflate stream


