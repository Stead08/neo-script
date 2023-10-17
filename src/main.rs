use std::fs;
use neo_script::runtime::runtime;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("[USAGE] neoscript file.ns")
    }
    // ファイルを開く
    let filename = &args[1];
    let src = fs::read_to_string(filename)?;
    runtime::run(&src)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        runtime::run("a > 3").unwrap();
    }
}
