use opcode::{Opcode, Code};

mod opcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("{}", &args[1]);

    let data = std::fs::read(&args[1])?;

    // let data_str = std::str::from_utf8(&data).unwrap();
    let code = Code::from(data)?;
    println!("{:?}", code.instrs);

    Ok(())
}
