//Julien Marcuse's code
use std::fs;
use crate::model::chartree::CharTree;

pub mod model;

//Frequency analysis on the number of tokens in every line
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect::<Vec<_>>();
    let file = args[0].clone();
    let contents = fs::read_to_string(file).unwrap();
    let mut tree = CharTree::new();
    let loaded = do_flag(&args, "-l", |path| {
        let contents = fs::read_to_string(path).unwrap();
        tree = CharTree::from_string(contents);
    });
    if !loaded {
        tree.train(contents.clone());
    }
    let mut saved = do_flag(&args, "-s", |path| {
       fs::write(path, tree.to_string()).unwrap();
    });
    if args.contains(&"-a".to_string()) {
        saved = true;
        contents.lines()
            .map(|line| tree.get_weirdness(&line))
            .map(|weirdness| average(&weirdness)).enumerate()
            .for_each(|(line, average)| println!("{}: {}", line + 1, average));
    }
    if !saved {
        println!("{:?}", tree.get_weirdness(&contents));
    }
}

fn average(nums: &Vec<u32>) -> f32 {
    nums.iter().sum::<u32>() as f32 / nums.len() as f32
}

fn do_flag(args: &Vec<String>, flag: &str, callback: impl FnOnce(&str) -> ()) -> bool {
    for i in 0..args.len() {
        if args[i] == flag {
            callback(&args[i + 1]);
            return true;
        }
    }
    false
}

