#[derive(Clone, Debug)]
enum TaggedValue {
    Int(i32),
    Bool(bool),
}

#[derive(Clone, Debug)]
enum TaggedNode {
    Value(TaggedValue),
    Add(Box<TaggedNode>, Box<TaggedNode>),
    IfStmt(Box<TaggedNode>, Box<TaggedNode>, Box<TaggedNode>), // cond and values
}

impl TaggedNode {
    fn int(x: i32) -> Box<Self> {
        Box::new(TaggedNode::Value(TaggedValue::Int(x)))
    }

    fn bool(b: bool) -> Box<Self> {
        Box::new(TaggedNode::Value(TaggedValue::Bool(b)))
    }

    fn add(a: Box<TaggedNode>, b: Box<TaggedNode>) -> Box<Self> {
        Box::new(TaggedNode::Add(a, b))
    }

    fn if_stmt(c: Box<TaggedNode>, a: Box<TaggedNode>, b: Box<TaggedNode>) -> Box<Self> {
        Box::new(TaggedNode::IfStmt(c, a, b))
    }

    fn eval(&self) -> TaggedValue {
        match &*self {
            Self::Value(v) => v.clone(),
            Self::Add(a, b) => {
                let TaggedValue::Int(a_eval) = a.eval() else { unreachable!("can only add ints!") };
                let TaggedValue::Int(b_eval) = b.eval() else { unreachable!("can only add ints!") };
                TaggedValue::Int(a_eval + b_eval)
            }
            Self::IfStmt(c, a, b) => {
                let TaggedValue::Bool(c_eval) = c.eval() else { unreachable!("only bools can be the condition!")};
                let a_eval = a.eval();
                let b_eval = b.eval();
                if c_eval {
                    a_eval
                } else {
                    b_eval
                }
            }
        }
    }
}

fn tagged_ast() -> Box<TaggedNode> {
    type T = TaggedNode;
    T::if_stmt(
        T::if_stmt(T::bool(false), T::bool(false), T::bool(true)),
        T::add(T::int(10), T::int(1)),
        T::int(-3),
    )
}

#[derive(Debug)]
struct Ast<T>(T);

trait Interp {
    type Repr<T>;

    fn ast<T>(ast: Self::Repr<T>) -> Self::Repr<Ast<T>>;
    fn bool(b: bool) -> Self::Repr<bool>;
    fn int(a: i32) -> Self::Repr<i32>;
    fn add(a: Self::Repr<i32>, b: Self::Repr<i32>) -> Self::Repr<i32>;
    fn if_stmt<T>(c: Self::Repr<bool>, a: Self::Repr<T>, b: Self::Repr<T>) -> Self::Repr<T>;
}

struct Eval;

impl Interp for Eval {
    type Repr<T> = T;

    fn ast<T>(ast: Self::Repr<T>) -> Self::Repr<Ast<T>> {
        Ast(ast)
    }
    fn bool(b: bool) -> Self::Repr<bool> {
        b
    }
    fn int(a: i32) -> Self::Repr<i32> {
        a
    }
    fn add(a: Self::Repr<i32>, b: Self::Repr<i32>) -> Self::Repr<i32> {
        a + b
    }
    fn if_stmt<T>(c: Self::Repr<bool>, a: Self::Repr<T>, b: Self::Repr<T>) -> Self::Repr<T> {
        if c {
            a
        } else {
            b
        }
    }
}

struct Display;

impl Interp for Display {
    type Repr<T> = String;

    fn ast<T>(ast: Self::Repr<T>) -> Self::Repr<Ast<T>> {
        ast
    }
    fn bool(b: bool) -> Self::Repr<bool> {
        format!("{b}")
    }
    fn int(a: i32) -> Self::Repr<i32> {
        format!("{a}")
    }
    fn add(a: Self::Repr<i32>, b: Self::Repr<i32>) -> Self::Repr<i32> {
        format!("({a}) + ({b})")
    }
    fn if_stmt<T>(c: Self::Repr<bool>, a: Self::Repr<T>, b: Self::Repr<T>) -> Self::Repr<T> {
        format!("if ({c}) then {{ {a} }} else {{ {b} }}")
    }
}

fn idx2ident(mut number: usize) -> String {
    let mut result = String::new();
    let base = 'a' as u8;
    while number > 0 {
        number -= 1; // Adjust number to start from 0 instead of 1
        let digit = (number % 26) as u8;
        result.insert(0, (base + digit) as char);
        number /= 26;
    }
    result
}

type Fun<D, C> = Box<dyn Fn(D) -> C>;

struct CCodegen;

impl Interp for CCodegen {
    type Repr<T> = Fun<usize, (String, String)>;

    fn ast<T>(ast: Self::Repr<T>) -> Self::Repr<Ast<T>> {
        Box::new(move |i| {
            let (ast, ident) = ast(i);
            let out = format!("#include <stdbool.h>\n#include <stdio.h>\nint main() {{\n{ast}printf(\"%d\\n\", {ident});\nreturn 0;\n}}");
            (out, String::new())
        })
    }
    fn bool(b: bool) -> Self::Repr<bool> {
        Box::new(move |i| {
            let ident = idx2ident(i);
            (format!("bool {ident} = {b};\n"), ident)
        })
    }
    fn int(a: i32) -> Self::Repr<i32> {
        Box::new(move |i| {
            let ident = idx2ident(i);
            (format!("int {ident} = {a};\n"), ident)
        })
    }
    fn add(a: Self::Repr<i32>, b: Self::Repr<i32>) -> Self::Repr<i32> {
        Box::new(move |i| {
            let ident = idx2ident(i);
            let (a, a_ident) = a(i.pow(2));
            let (b, b_ident) = b(i.pow(3));
            let out = format!("{a}{b}int {ident} = {a_ident} + {b_ident};\n");
            (out, ident)
        })
    }
    fn if_stmt<T>(c: Self::Repr<bool>, a: Self::Repr<T>, b: Self::Repr<T>) -> Self::Repr<T> {
        Box::new(move |i| {
            let ident = idx2ident(i);
            let (c, c_ident) = c(i.pow(5));
            let (a, a_ident) = a(i.pow(7));
            let (b, b_ident) = b(i.pow(11));
            let out = format!("{c}int {ident};\nif ({c_ident}) {{\n{a}{ident} = {a_ident};\n}} else {{\n{b}{ident} = {b_ident};\n}}\n");
            (out, ident)
        })
    }
}

fn tagless_ast<I: Interp>() -> I::Repr<Ast<i32>> {
    I::ast(I::if_stmt(
        I::if_stmt(I::bool(false), I::bool(false), I::bool(true)),
        I::add(I::int(10), I::int(1)),
        I::int(-1),
    ))
}

fn main() {
    // let tagged_ast = tagged_ast();
    // // dbg!(&tagged_ast);
    // let eval = tagged_ast.eval();
    // dbg!(eval);

    // let tagless_eval = tagless_ast::<Eval>();
    // dbg!(tagless_eval);

    // let tagless_disp = tagless_ast::<Display>();
    // dbg!(tagless_disp);

    let c = tagless_ast::<CCodegen>()(2).0;
    println!("\n\n{c}");
}
