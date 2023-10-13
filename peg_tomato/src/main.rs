use peg_tomato::runner;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("[USAGE] peg_tomato file.tomato")
    }
    // ファイルを開く
    let filename = &args[1];
    let src = fs::read_to_string(filename)?;
    runner::run(&src)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        assert_eq!(runner::run("print 32").unwrap(), 32);
        assert_eq!(runner::run("print 1+2*3").unwrap(), 7);
        assert_eq!(
            runner::run("a = 3; if a == 3 {print 1} else {print 0}").unwrap(),
            1
        );
        assert_eq!(
            runner::run("a = 0; for i = 1 to 10 { a = a + i}; print a").unwrap(),
            55
        );
        assert_eq!(runner::run("print \"abc\"").unwrap(), 0);
    }
}
