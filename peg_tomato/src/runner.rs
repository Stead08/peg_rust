use crate::node::Node;
use crate::parser::tomato;
use std::collections::HashMap;

// プログラム全体で使うコンテキストを定義
struct Context {
    // 変数と値を保持
    vars: HashMap<String, i64>,
}

impl Context {
    // コンテキストを作成
    fn new() -> Context {
        Context {
            vars: HashMap::new(),
        }
    }
}

// 構文木を一つ実行する
fn run_node(ctx: &mut Context, node: Node) -> i64 {
    match node {
        Node::Number(v) => v,
        Node::Calc(op, l, r) => {
            //計算式
            calc_op(op, run_node(ctx, *l), run_node(ctx, *r))
        }
        Node::GetVar(name) => {
            //変数の値を得る
            match ctx.vars.get(&name) {
                Some(v) => *v,
                None => 0,
            }
        }
        Node::SetVar(name, node) => {
            //変数の代入
            let val = run_node(ctx, *node);
            ctx.vars.insert(name, val);
            val
        }
        Node::If(cond, true_n, false_n) => {
            let cond_v = run_node(ctx, *cond);
            if cond_v > 0 {
                run_nodes(ctx, &true_n)
            } else {
                run_nodes(ctx, &false_n)
            }
        }
        Node::For(name, start, end, body) => {
            // for文
            let mut r = 0;
            let nodes = body;
            for i in start..=end {
                ctx.vars.insert(name.clone(), i);
                r = run_nodes(ctx, &nodes);
            }
            r
        }
        Node::PrintStr(v) => {
            println!("{}", v);
            0
        }
        Node::Print(node) => {
            //print文
            let v = run_node(ctx, *node);
            println!("{}", v);
            v
        }
        _ => 0,
    }
}

// 演算子に基づくて計算を行う
fn calc_op(op: char, val_l: i64, val_r: i64) -> i64 {
    match op {
        '+' => val_l + val_r,
        '-' => val_l - val_r,
        '*' => val_l * val_r,
        '/' => val_l / val_r,
        '%' => val_l % val_r,
        '=' => {
            if val_l == val_r {
                1
            } else {
                0
            }
        }
        '!' => {
            if val_l != val_r {
                1
            } else {
                0
            }
        }
        '>' => {
            if val_l > val_r {
                1
            } else {
                0
            }
        }
        'g' => {
            if val_l >= val_r {
                1
            } else {
                0
            }
        }
        '<' => {
            if val_l < val_r {
                1
            } else {
                0
            }
        }
        'l' => {
            if val_l <= val_r {
                1
            } else {
                0
            }
        }
        _ => {
            if val_l <= val_r {
                1
            } else {
                0
            }
        }
    }
}

// 繰り返しNodeを実行
fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> i64 {
    let mut result = 0;
    nodes.iter().for_each(|node| {
        result = run_node(ctx, node.clone());
    });
    result
}

// 手軽にプログラムを実行する関数
pub fn run(src: &str) -> anyhow::Result<i64> {
    let nodes = tomato::parse(src)?;
    let mut ctx = Context::new();
    Ok(run_nodes(&mut ctx, &nodes))
}
