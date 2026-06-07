use std::fs;
use std::path::Path;
use colored::*;

pub fn create_architecture_template(project_name:&str){
    println!("{} {}","Creating CAOS in project".blue().bold(),project_name);
    let sub_directories = [
        "src/domain/entities",
        "src/domain/dtos",
        "src/domain/datasource",
        "src/domain/repositories",
        "src/infrastructure/external",
        "src/infrastructure/persistence",
        "src/application/use-cases",
        "src/application/services",
        "src/infrastructure/database",
        "src/presentation/controllers",
        "src/presentation/routers",
        "src/presentation/middlewares"
    ];
    for sub_dir in sub_directories.iter(){
        let full_path = Path::new(project_name).join(sub_dir);
        match fs::create_dir_all(&full_path) {
                    Ok(_) => println!(
                        "{} directory {} created!", 
                        "Success:".green().bold(), 
                        full_path.display().to_string().yellow()
                    ),
                    Err(err) => println!(
                        "{} creating directory '{}': {}", 
                        "Error".red().bold(), 
                        full_path.display(), 
                        err
                    ),
                }
    }
    println!("\n{}", "¡CAOS ordenó tu proyecto con éxito! 🚀".green().bold());
}