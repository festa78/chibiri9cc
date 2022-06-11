use super::tokenize;

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Eq,
    Ne,
    Lt,
    Le,
    Num,
}

pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: Option<i32>,
}

fn new_node(kind: NodeKind) -> Node {
    Node {
        kind,
        lhs: None,
        rhs: None,
        val: None,
    }
}

fn new_binary(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    let mut node = new_node(kind);
    node.lhs = Some(Box::new(lhs));
    node.rhs = Some(Box::new(rhs));
    node
}

fn new_unary(kind: NodeKind, expr: Node) -> Node {
    let mut node = new_node(kind);
    node.lhs = Some(Box::new(expr));
    node
}

fn new_num(val: i32) -> Node {
    let mut node = new_node(NodeKind::Num);
    node.val = Some(val);
    node
}

// expr = equality
pub fn expr(token: &mut tokenize::Token) -> Result<Node, Box<dyn std::error::Error>> {
    equality(token)
}

// equality = relation ("==" relational | "!=" relational)*
fn equality(token: &mut tokenize::Token) -> Result<Node, Box<dyn std::error::Error>> {
    let mut node = relational(token)?;

    loop {
        if tokenize::consume_ops(token, tokenize::ReservedKind::Equal) {
            node = new_binary(NodeKind::Eq, node, relational(token)?);
        } else if tokenize::consume_ops(token, tokenize::ReservedKind::NotEqual) {
            node = new_binary(NodeKind::Ne, node, relational(token)?);
        } else {
            break;
        }
    }
    Ok(node)
}

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
fn relational(token: &mut tokenize::Token) -> Result<Node, Box<dyn std::error::Error>> {
    let mut node = add(token)?;

    loop {
        if tokenize::consume_ops(token, tokenize::ReservedKind::Less) {
            node = new_binary(NodeKind::Lt, node, add(token)?);
        } else if tokenize::consume_ops(token, tokenize::ReservedKind::LessEqual) {
            node = new_binary(NodeKind::Le, node, add(token)?);
        } else if tokenize::consume_ops(token, tokenize::ReservedKind::Larger) {
            node = new_binary(NodeKind::Lt, add(token)?, node);
        } else if tokenize::consume_ops(token, tokenize::ReservedKind::LargerEqual) {
            node = new_binary(NodeKind::Le, add(token)?, node);
        } else {
            break;
        }
    }
    Ok(node)
}

// add = mul ("+" mul | "-" mul)*
pub fn add(token: &mut tokenize::Token) -> Result<Node, Box<dyn std::error::Error>> {
    let mut node = mul(token)?;

    loop {
        if tokenize::consume_ops(token, tokenize::ReservedKind::Plus) {
            node = new_binary(NodeKind::Add, node, mul(token)?);
        } else if tokenize::consume_ops(token, tokenize::ReservedKind::Minus) {
            node = new_binary(NodeKind::Sub, node, mul(token)?);
        } else {
            break;
        }
    }
    Ok(node)
}

// mul = unary ("*" unary | "/" unary)*
fn mul(token: &mut tokenize::Token) -> Result<Node, Box<dyn std::error::Error>> {
    let mut node = unary(token)?;

    loop {
        if tokenize::consume_ops(token, tokenize::ReservedKind::Mul) {
            node = new_binary(NodeKind::Mul, node, unary(token)?);
        } else if tokenize::consume_ops(token, tokenize::ReservedKind::Div) {
            node = new_binary(NodeKind::Div, node, unary(token)?);
        } else {
            break;
        }
    }
    Ok(node)
}

// unary = ("+" | "-")? unary
fn unary(token: &mut tokenize::Token) -> Result<Node, Box<dyn std::error::Error>> {
    if tokenize::consume_ops(token, tokenize::ReservedKind::Plus) {
        return unary(token);
    }
    if tokenize::consume_ops(token, tokenize::ReservedKind::Minus) {
        return Ok(new_unary(NodeKind::Neg, unary(token)?));
    }
    primary(token)
}

// primary = "(" expr ")" | num
fn primary(token: &mut tokenize::Token) -> Result<Node, Box<dyn std::error::Error>> {
    if tokenize::consume_ops(token, tokenize::ReservedKind::ParenLeft) {
        let node = expr(token)?;
        tokenize::expect_ops(token, tokenize::ReservedKind::ParenRight)?;
        return Ok(node);
    }

    Ok(new_num(tokenize::expect_number(token)?))
}
