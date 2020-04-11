use std::rc::Rc;

#[derive(Clone, Copy)] struct Element;
#[derive(Clone)] struct Label(String);

#[derive(Clone)]
enum Value {
    Unit,
    Inl(Rc<Value>),
    Inr(Rc<Value>),
    Pair(Rc<Value>, Rc<Value>),
    Prim(Rc<Value>),
    Id(Element),
}
// #[derive(PartialEq)]
#[derive(Clone)]
enum Type {
    Zero,
    One,
    Sum(Rc<Type>, Rc<Type>),
    Product(Rc<Type>, Rc<Type>),
    Prim,
    Lbl(Label)
}

// fn lambda(_e: &Element) -> Label {
//     Label("".to_string())
// }

// fn upsilon(_e: &Element) -> Value {
//     Value::Unit
// }

// fn sigma(_l: Label) -> Type {
//     Type
// }

// fn tau(_v: Value) -> Type {
//     Type
// }

fn main() {
    // let e = Element;
    // if sigma(lambda(&e)) == tau(upsilon(&e)) {
    //     println!("Algebraic Property Graph!");
    // } else {
    //     println!("God Damn!");
    // }

    let v1 = Element;
    let v2 = Element;
    let e1 = Element;

    let person_label = Label("Person".to_string());
    let knows_label = Label("knows".to_string());

    let know_e1 = Value::Pair(Rc::new(Value::Id(v1)), Rc::new(Value::Id(v2)));

    let person_type = Rc::new(Type::Lbl(person_label.clone()));
    let knows_type = Type::Product(person_type.clone(), person_type.clone());

    let c1 = (v1, person_label.clone(), Value::Unit, Type::One);
    let c2 = (v2, person_label.clone(), Value::Unit, Type::One);
    let c3 = (e1, knows_label, know_e1, knows_type);
}
