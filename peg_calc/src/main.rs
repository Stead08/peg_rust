peg::parser!( grammar calc() for str {
    // ルートとなる規則
    pub rule eval() -> i64
    = expr()

    // 足し算と引き算を行うルールを追加
    rule expr() -> i64
    = l:term() "+" r:term() { l + r }
    / l:term() "-" r:term() { l - r }
    / term()

    // 掛け算と割り算を行うルールを追加
    rule term() -> i64
    = l:value() "*" r:term() { l * r }
    / l:value() "/" r:term() { l / r }
    / v:value()

    // 値を読むルールを追加
    rule value() -> i64
    = number()
    / "(" v:expr() ")" { v }

    // 数値を取得するルールを指定
    rule number() -> i64
    = n:$(['0'..='9']+)
    { n.parse().unwrap() }
});


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Please input calculation");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim_end();
    let result = calc::eval(input)?;
    println!("{}", result);
    Ok(())

}

// test
#[cfg(test)]
mod tests {
    use super::*;
    // 四則演算子の単体テスト
    #[test]
    fn test_add() {
        assert_eq!(calc::eval("1+2").unwrap(), 3);
    }
    #[test]
    fn test_sub() {
        assert_eq!(calc::eval("1-2").unwrap(), -1);
    }
    #[test]
    fn test_mul() {
        assert_eq!(calc::eval("2*3").unwrap(), 6);
    }
    #[test]
    fn test_div() {
        assert_eq!(calc::eval("4/2").unwrap(), 2);
    }
    // 括弧のテスト
    #[test]
    fn test_paren() {
        assert_eq!(calc::eval("(1+2)*3").unwrap(), 9);
    }

}

