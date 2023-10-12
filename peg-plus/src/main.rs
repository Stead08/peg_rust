peg::parser!( grammar calc() for str {
    // ルートとのある規則を追加
    pub rule eval() -> i64 //規則の名前
    = term() // 構文定義

    //足し算を行う規則を追加
    rule term() -> i64
    = v1:num() "+" v2:num()
    { v1 + v2}

    //数値を読む規則を追加
    rule num() -> i64
    = value:$(['0' ..='9']+)
    {value.parse().unwrap()}

});
fn main() {
    //足し算の計算式を実行
    println!("2+5={}", calc::eval("2+5").unwrap());
    println!("2+5={}", calc::eval("8+2").unwrap());
    println!("2+5={}", calc::eval("200+50").unwrap());
}
