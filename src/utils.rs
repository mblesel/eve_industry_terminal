use yaml_rust::{YamlLoader, Yaml, YamlEmitter};
use std::io::{self,Write};
use std::fs;

pub fn load_yaml(filepath: &str) -> Vec<Yaml>
{
    let yaml_file = fs::read_to_string(filepath)
        .expect("Cannot read yaml file");
    YamlLoader::load_from_str(&yaml_file)
        .expect("Cannot deserialize yaml file")
}

pub fn _dump_yaml(yaml_obj: &Yaml)
{
    let mut out = String::new();
    let mut emitter = YamlEmitter::new(&mut out);
    emitter.dump(yaml_obj).unwrap();
    println!("{}", out);
}

pub fn read_input(prompt: &str) -> String
{
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn parse_input<T>(prompt: &str, min: T, max: T) -> T
    where T: std::str::FromStr + Default + std::cmp::PartialOrd,
{
    loop
    {
        print!("{}",prompt);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        let ret = buffer.trim().parse::<T>().unwrap_or_default();

        if (ret >= min) & (ret <= max)
        {
            return ret
        }
    }
}
