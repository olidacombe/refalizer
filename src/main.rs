use color_eyre::Result;
use refalizer::to_branch_name;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input = args.join(" ");
    let branch_name = to_branch_name(input)?;
    println!("{branch_name}");
    Ok(())
}
