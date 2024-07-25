use std::{fs::File, io::Write, path::Path};

fn compile(input: &String) -> Result<(), std::io::Error> {
    let mut assembly = String::new();
    assembly.push_str(format!("movl ${}, %eax\n", input).as_str());
    assembly.push_str("ret\n");
    let output_path = Path::new("dump/assembly.s");
    let mut output_file = File::create(output_path)?;
    output_file.write_all(assembly.as_bytes())?;

    Ok(())
}

fn handle_args(args: &[String]) -> Option<&String> {
    if args.len() != 2 {
        eprintln!("The compiler currently only accepts a single argument which is either a file or a scheme expression");
        std::process::exit(1);
    }
    args.get(1)
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let input = handle_args(&args).expect("critical first argument exists but can't be read");
    compile(input)?;

    Ok(())
}
