use std::env;
use colored::*;

pub fn parse_arguments()-> Option<String>{
    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("{}", "Error: Please provide a project name".red().bold());
        println!("Uso: {} {}", "caos".cyan(), "<nombre_del_proyecto>".yellow());
            return None;
    }
    Some(args[1].clone())
}