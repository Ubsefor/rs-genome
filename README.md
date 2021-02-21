# rs-genome

A small tool to check the statements about genomes with some examples: [task](task.png)

# Building:

On Unix-like systems with rust available: `git clone https://github.com/Ubsefor/rs-genome ; cd rs-genome`

You need rust cargo to build the project. 

Follow the official installation instructions to get it: [guide](https://www.rust-lang.org/tools/install)

Next `cd` into cloned repository and run `cargo build --release`

# Running:

From the project's root directory:

`cargo run --release -- args` where args can be:

`-h`: prints usage message

`-v`: prints version

`<filepath>` – place a path to the file here, for the given examples you can just write the name of the file

Note, that the program accepts only files, containing raw genetic code (see [example](HID_NC_001802.1.txt) to get the idea), so if you download genomes from  [NCBI](https://www.ncbi.nlm.nih.gov), use FASTA format and remove the header message!



