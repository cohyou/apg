struct Entity;
struct Label;
struct Value;
#[derive(PartialEq)]struct Type;

fn lambda(_e: &Entity) -> Label {
    Label
}

fn upsilon(_e: &Entity) -> Value {
    Value
}

fn sigma(_l: Label) -> Type {
    Type
}

fn tau(_v: Value) -> Type {
    Type
}

fn main() {
    let e = Entity;
    if sigma(lambda(&e)) == tau(upsilon(&e)) {
        println!("Algebraic Property Graph!");
    } else {
        println!("God Damn!");
    }
}
