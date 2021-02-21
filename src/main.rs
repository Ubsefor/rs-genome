use std::env;
use std::fs;
use std::path::Path;

fn usage() {
    println!("Usage: rs-genome <genome file path>\nNote, that file must not contain header!");
}

fn version(){
    println!("version: 0.1.0");
}

pub fn count_genes(genome : String, gene : char) -> usize {
    return genome.matches(gene).count();
}

pub fn count_cg(genome : String) -> usize {
    return genome.matches("CG").count();
}

pub fn count_ta(genome : String) -> usize {
    return genome.matches("TA").count()
}

pub fn check_skew(genome : String) -> bool{
    let length = genome.len();
    let half = length / 2;
    let first_half = &genome[0..half];
    let second_half = &genome[(half + 1)..];
    let count_c = first_half.matches("C").count();
    let count_g = first_half.matches("G").count();
    let count_c_last = second_half.matches("C").count();
    let count_g_last = second_half.matches("G").count();
    return count_c > count_g && count_c_last < count_g_last;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            usage();
            return;
        },

        2 => {
            if  &args[1] == "-h" {
                usage();
                return;
            }
            if &args[1] == "-v"{
                version();
                return;
            }
            let filepath = &args[1];
            if !(Path::new(filepath).is_file()){
                panic!("Error: couldn't read file at specified filepath!");
            }
            let contents = fs::read_to_string(filepath)
                .expect("Something went wrong reading the file");
            if !(contents.chars().nth(0).unwrap() == 'A' 
                    || contents.chars().nth(0).unwrap() == 'C'
                    || contents.chars().nth(0).unwrap() == 'G' 
                    || contents.chars().nth(0).unwrap() == 'T') {
                panic!("Error: file seems to contain a header or not a raw genome!")
            }
            println!("Parsing genome in file: {}", filepath);
            println!("Total genome size: {}", contents.len());
            println!("Amount of A: {}", count_genes(contents.clone(), 'A'));
            println!("Amount of C: {}", count_genes(contents.clone(), 'C'));
            println!("Amount of T: {}", count_genes(contents.clone(), 'T'));
            println!("Amount of G: {}", count_genes(contents.clone(), 'G'));
            println!("Amount of CG: {}", count_cg(contents.clone()));
            println!("Amount of TA: {}", count_ta(contents.clone()));
            println!("GC Skew is present: {}", check_skew(contents));
            return ;
        },
        
        _ => {
            usage();
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts(){
        assert_eq!(count_genes("AAA".to_string(), 'A'), 3);
        assert_eq!(count_ta("TAAT".to_string()), 1);
        assert_eq!(count_cg("CGGC".to_string()), 1);
    }

    #[test]
    fn test_skew(){
        assert_eq!(check_skew("CCGG".to_string()), true);
    }
}

