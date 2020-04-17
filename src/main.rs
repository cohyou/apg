use std::rc::Rc;
use std::collections::{HashMap, HashSet};
// use std::cell::RefCell;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Element(String);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Value {
    Unit,
    Inl(Rc<Value>),
    Inr(Rc<Value>),
    Pair(Rc<Value>, Rc<Value>),
    Prim(Rc<Value>),
    Id(Rc<Element>),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Label(String);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Type {
    Zero,
    One,
    Sum(Rc<Type>, Rc<Type>),
    Product(Rc<Type>, Rc<Type>),
    Prim,
    Lbl(Rc<Label>),
}

struct APG {
    elements: HashSet<Rc<Element>>,
    values: HashSet<Rc<Value>>,    
    labels: HashSet<Rc<Label>>,
    types: HashSet<Rc<Type>>,

    lambda: fn(_e: &Element) -> Label,
    tau: fn(_v: Value) -> Type,
}

impl APG {
//     fn new(
//             es: HashSet<Element>, vs: HashSet<Value<'a>>,
//             ls: HashSet<Label>, ts: HashSet<Type<'a>>,
//             lambda: fn(_e: &Element) -> Label,
//             upsilon: fn(_e: &Element) -> Value,
//             sigma: fn(_l: Label) -> Type<'a>,
//             tau: fn(_v: Value) -> Type<'a>,        
//           ) -> APG<'a> {
//         APG {
//             elements: es, 
//             values: vs,
//             labels: ls,
//             types: ts,

//             lambda: lambda,
//             upsilon: upsilon,
//             sigma: sigma,
//             tau: tau,
//         }
//     }

    fn add_element(&mut self, name: &str) {
        let element = Element(name.to_string());
        self.elements.insert(Rc::new(element));
    }

    fn get_element(&self, name: &str) -> Option<Rc<Element>> {
        for element in self.elements.iter() {            
            if element.0 == name {
                return Some(element.clone());
            }            
        }

        None
    }

    fn add_value(&mut self, v: Value) {
        let v = Rc::new(v);
        self.values.insert(v.clone());
    }
    
    fn add_element_value(&mut self, element_name: &str) {
        let v = Value::Id(self.get_element(element_name).unwrap());
        self.add_value(v);
    }

    fn get_element_value(&self, element_name: &str) -> Option<Rc<Value>> {
        for value in self.values.iter() {            
            if let Ok(Value::Id(elem)) = Rc::try_unwrap(value.clone()) {
                if let Ok(e) = Rc::try_unwrap(elem) {
                    if e.0 == element_name {
                        return Some(value.clone());
                    }
                }                
            }            
        }

        None        
    }

    fn add_pair_value(&mut self, v1: Rc<Value>, v2: Rc<Value>) {
        self.add_value(Value::Pair(v1, v2));
    }

    fn add_label(&mut self, name: &str) {
        let lbl = Label(name.to_string());
        self.labels.insert(Rc::new(lbl));
    }

    fn get_label(&self, name: &str) -> Option<Rc<Label>> {
        for label in self.labels.iter() {            
            if label.0 == name {
                return Some(label.clone());
            }            
        }

        None
    }

    fn add_type(&mut self, tp: Type) {
        let tp = Rc::new(tp);
        self.types.insert(tp.clone());
    }

    fn add_label_type(&mut self, label_name: &str) {
        let tp = Type::Lbl(self.get_label(label_name).unwrap());
        self.add_type(tp);
    }

    fn get_label_type(&self, label_name: &str) -> Option<Rc<Type>> {
        for tp in self.types.iter() {            
            if let Ok(Type::Lbl(label)) = Rc::try_unwrap(tp.clone()) {
                if let Ok(l) = Rc::try_unwrap(label) {
                    if l.0 == label_name {
                        return Some(tp.clone());
                    }
                }                
            }            
        }

        None        
    }

    fn add_product_type(&mut self, tp1: Rc<Type>, tp2: Rc<Type>) {
        self.types.insert(Rc::new(Type::Product(tp1, tp2)));
    }
}

impl Default for APG {
    fn default() -> APG {
        APG {
            elements: HashSet::default(),
            values: HashSet::default(), 
            labels: HashSet::default(),
            types: HashSet::default(),

            lambda: |_e| Label("".to_string()),
            tau: |_e| Type::Zero,
        }
    }
}

fn main() {
    let mut apg = APG::default();

    // set up elements
    apg.add_element("v1");
    apg.add_element("v2");
    apg.add_element("e1");

    // set up values
    apg.add_value(Value::Unit);
    apg.add_element_value("v1");
    apg.add_element_value("v2");
    apg.add_pair_value(apg.get_element_value("v1").unwrap(), apg.get_element_value("v2").unwrap());

    // set up label
    apg.add_label("Person");
    apg.add_label("knows");

    // set up type
    apg.add_type(Type::One);
    apg.add_label_type("Person");
    apg.add_product_type(apg.get_label_type("Person").unwrap(), apg.get_label_type("Person").unwrap());
 

    // let _c1 = (v1, person_label.clone(), Value::Unit, Type::One);
    // let _c2 = (v2, person_label.clone(), Value::Unit, Type::One);
    // let _c3 = (e1, knows_label.clone(), know_e1.clone(), knows_type);
}
