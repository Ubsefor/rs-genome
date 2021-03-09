# rs-genome

![](https://github.com/Ubsefor/rs-genome/workflows/Rust/badge.svg)

A small tool to check the statements about genomes with some examples: [task](task.png)

# Building:

You need Rust's Cargo to build the project. 

Follow the official installation instructions to get it: [guide](https://www.rust-lang.org/tools/install)

You also need GCC or any other cc-compliant linker to build the project. I use GCC: [installation](https://gcc.gnu.org/install/)

(OPTIONAL) You also need to install git-lfs for ncbi-genomes to be pulled from the repo automatically. See the [wiki](https://github.com/git-lfs/git-lfs/wiki/Installation). If you do not wish to install git-lfs, you can simply download files directly from the github. Mind the file encoding as Windows uses CR LF line breaks style, while unix-like systems commonly use LF!

On Unix-like systems with rust available: 
`git clone https://github.com/Ubsefor/rs-genome ; cd rs-genome ; cargo build --release`

# Running:

From the project's root directory:

`cargo run --release -- args` where args can be:

`-h`: prints usage message

`-v`: prints version

`<filepath>` – place a path to the file here, for the given examples you can just write the name of the file

Note, that the program accepts only files, containing raw genetic code (see [example](HID_NC_001802.1.txt) to get the idea), so if you download genomes from  [NCBI](https://www.ncbi.nlm.nih.gov), use FASTA format and remove the header message!

# Testing:

You can also run `cargo test` mainly to see if your rust installation is working properly.

