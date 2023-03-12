
use std::process::Command;

use execute::Execute;
use eyre::Result;

pub fn format_file(file : &str) -> Result<()>{
    const BIN_PATH : &str = "/Users/Antoine/Code/cairo1/cairo-lang/cairo/target/release/cairo-format" ;
    let mut command = Command::new(BIN_PATH);
    command.arg("--check");
    command.arg(file);

    let output = command.execute_output().unwrap();

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }
    Ok(())
}
