#[derive(Debug, Clone)]
pub enum Node {
    /** 何もしない **/
    Nop,
    /** 数値を表す**/
    Number(i64),
    /** 計算を表す **/
    Calc(char, Box<Node>, Box<Node>),
    /** If文 **/
    If(Box<Node>, Vec<Node>, Vec<Node>),
    /** for文 **/
    For(String, i64, i64, Vec<Node>),
    /** print文(計算出力)**/
    Print(Box<Node>),
    /** print文(定数出力) **/
    PrintStr(String),
    /** 変数代入 **/
    SetVar(String, Box<Node>),
    /** 変数参照 **/
    GetVar(String),
}

impl Node {
    // 手軽にNode::Calc型を返す
    pub fn calc(op: char, l: Node, r: Node) -> Node {
        Node::Calc(op, Box::new(l), Box::new(r))
    }
    // 手軽にNode::If型を返す関数
    pub fn if_(cond: Node, t: Vec<Node>, f: Vec<Node>) -> Node {
        Node::If(Box::new(cond), t, f)
    }
}
