[![Anaconda-Server Badge](https://img.shields.io/badge/install%20with-bioconda-brightgreen.svg?style=flat)](https://bioconda.github.io/recipes/fastlin/README.html) 
[![Crates.io](https://img.shields.io/crates/v/fastlin)](https://crates.io/crates/fastlin)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/rderelle/fastlin)](https://github.com/rderelle/fastlin/releases)

<h1 align="center">fastlin</h1>


### Overview

Fastlin is an ultra-fast program to perform lineage typing of <i>Mycobacterium tuberculosis</i> complex (MTBC) FASTQ read data, BAM files and FASTA assemblies. Using the split-kmer approach, it can accuratly predict MTBC lineages and strain mixtures in seconds.

Reference: [fastlin: an ultra-fast program for Mycobacterium tuberculosis complex lineage typing.](https://doi.org/10.1093/bioinformatics/btad648)

Main updates since publication:
+ 0.2.3 : FASTA files as input (also using [seq_io](https://github.com/markschl/seq_io))
+ 0.3.0 : multi-threading available
+ 0.4.0 : BAM files as input (using [rust-htslib](https://github.com/rust-bio/rust-htslib))

There is no planned updates at the moment. Please open an issue if you have any suggestion or request.

### Installation
To install fastlin via cargo (you must have the [rust toolchain](https://www.rust-lang.org/tools/install) installed):
```
cargo install fastlin
```
Or you can download the latest release from this repository and compile it using cargo:
```
cargo install --path directory_release
```
Alternatively, you can install precompiled binaries using Conda:
```
conda install -c bioconda fastlin
```
You will also need a barcode file (see Input files below).

### Running fastlin
The default command line is:
```
fastlin -d your_directory -b your_barcodes.txt
```
If your dataset consists of FASTQ files that are not BAM-derived, then you can apply a maximum kmer coverage threshold to reduce runtimes: 
```
fastlin -d your_directory -b your_barcodes.txt -x 80
```

### Input files
<p>Fastlin takes as input the path of the directory containing FASTQ, BAM and/or FASTA files. The directory can contain a mix of FASTA geome assemblies, BAM alignment files, paired-end and single-end FASTQ files. FASTQ and FASTA files should be gzipped, with the following extensions:</p>

- **.fastq.gz** or **.fq.gz** for FASTQ read data. The names of paired-end files should be in the form name_1.fq.gz and name_2.fq.gz (or equivalent with fastq.gz)
- **.bam**, or **.BAM** for BAM files. Here the maximum kmer coverage is ignored since BAM files can be sorted.
- **.fas.gz**, **.fasta.gz** or **.fna.gz** for FASTA genome assemblies. Here, minimum occurence is set to 1 and the maximum kmer coverage is ignored.

<p>Please note that BAM files are analyzed in the same way as FASTQ files, by scanning reads without considering quality scores.
You may find faster scripts or programs that focus solely on specific genomic positions.</p> 

<p>The MTBC barcode file can be downloaded from https://www.github.com/rderelle/barcodes-fastlin. 
Alternatively, you can build and test your own kmer barcodes using the Python scripts available in that directory.</p> 

### Manual

A full description of fastlin parameters can be found [here](https://github.com/rderelle/fastlin/blob/main/parameters.md).

### Output file
Fastlin output consists of a tab-delimited file with the following fields:
+ sample: sample name
+ data type: 'assembly', 'BAM', 'single' (reads) or 'paired' (-end reads)
+ k_cov: theoretical kmer coverage of the fastq files(s) based on the number of extracted kmers
+ mixture: pure ('no') or mixed ('yes') sample
+ lineages: detected lineages (median kmer occurences within paratheses)
+ log_barcodes: kmer barcodes passing the minimum occurence threshold, indicated by their kmer occurence and grouped by lineages

Here is a simple example:
> #sample&nbsp;&nbsp;&nbsp;&nbsp;data type&nbsp;&nbsp;&nbsp;&nbsp;k_cov&nbsp;&nbsp;&nbsp;&nbsp;mixture&nbsp;&nbsp;&nbsp;&nbsp;lineages&nbsp;&nbsp;&nbsp;&nbsp;log_barcodes&nbsp;&nbsp;&nbsp;&nbsp;log_errors  
ERRxxxxx&nbsp;&nbsp;&nbsp;&nbsp;paired&nbsp;&nbsp;&nbsp;&nbsp;118&nbsp;&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;2 (45)&nbsp;&nbsp;&nbsp;&nbsp;2 (42, 48, 39, 43, 54, 47, 45), 4.1 (4)

The sample ERRxxxxx contains a single strain belonging to lineage 2. This typing is supported by 7 kmer barcodes, with a median number of occurences of 45. Since the abundance of the strain is far below the theoretical kmer coverage (equal here to 118), we can conclude that the sample is likely to contain high level of contaminations or sequencing errors.

### Multi-threading
<p>By default, fastlin runs on 1 thread. The number of threads can be increased using the '-t' parameter, which will split the sample set among all threads (for a single sample, increasing the number of threads will have no impact on runtime).</p>

<p>Here are some examples of runtimes (in seconds) using real-world Mtb genomic data on a M2 Macbook Air:</p>
<div align="center">

| data               | 1 thread  | 4 threads |
|--------------------|-----------|-----------|
| 12 paired FASTQ    |   66.9    | 19.3      |
| 4 BAM files        |   26.9    | 9.4       |
| 190 genomes FASTA  |   6.7     | 1.8       |

</div>

### Error handling
<p>When fastlin cannot read a fastq file (e.g., faulty record within the fastq file, corrupt gzip file), it stops scanning it, re-initialises all values to 0 and reports the error message in the last column of the output file. Here is an example of output with 3 different errors:</p>

> #sample&nbsp;&nbsp;&nbsp;&nbsp;data type&nbsp;&nbsp;&nbsp;&nbsp;k_cov&nbsp;&nbsp;&nbsp;&nbsp;mixture&nbsp;&nbsp;&nbsp;&nbsp;lineages&nbsp;&nbsp;&nbsp;&nbsp;log_barcodes&nbsp;&nbsp;&nbsp;&nbsp;log_errors  
dummy1&nbsp;&nbsp;&nbsp;single&nbsp;&nbsp;&nbsp;0&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Error in file "reads/dummy1.fastq.gz": FASTQ parse error: sequence length is 150, but quality length is 50 (record 'ERR551806.5' at line 17).  
dummy2&nbsp;&nbsp;&nbsp;single&nbsp;&nbsp;&nbsp;0&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Error in file "reads/dummy2.fastq.gz": invalid gzip header  
dummy3&nbsp;&nbsp;&nbsp;single&nbsp;&nbsp;&nbsp;0&nbsp;&nbsp;&nbsp;no&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Error in file "reads/dummy3.fastq.gz": corrupt deflate stream
