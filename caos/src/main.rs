mod args;
mod generator;


fn main(){
    if let Some(project_name) = args::parse_arguments(){
        generator::create_architecture_template(&project_name);
    }
}