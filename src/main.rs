use std::rc::Rc;
use std::collections::{HashMap, HashSet};
// use std::cell::RefCell;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Element(String);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Value {
    Unit,
    Inl(Rc<Value>, Rc<Type>),
    Inr(Rc<Type>, Rc<Value>),
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
    lambda_upsilon: HashMap<String, (String, Rc<Value>)>,
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

    fn add_lambda_upsilon(&mut self, e: &str, l: &str, v: Value) {
        self.add_element(e);
        self.add_label(l);
        let v_rc = Rc::new(v);
        self.values.insert(v_rc.clone());
        self.lambda_upsilon.insert(e.to_string(), (l.to_string(), v_rc.clone()));
    }

    fn check_labels(&self) {
        for element in self.elements.iter() {
            println!("{:?}", element);
        }    
    }
}

impl Default for APG {
    fn default() -> APG {
        APG {
            elements: HashSet::default(),
            values: HashSet::default(), 
            labels: HashSet::default(),
            lambda_upsilon: HashMap::default(),
        }
    }
}

struct APGMorphism {
    from: Rc<APG>,
    to: Rc<APG>,
    element_mapping: fn(Rc<Element>) -> Rc<Element>,
}

impl APGMorphism {
fn check_source_labels(&self) {
    self.from.check_labels();
}
}

macro_rules! ev {
    ($apg: ident, $e: expr) => {
        $apg.get_element_value($e).unwrap()
    };
}

macro_rules! add {
    ($apg: ident, $e: expr, $l: expr, ()) => {
        $apg.add_lambda_upsilon($e, $l, Value::Unit)
    };
    ($apg: ident, $e: expr, $l: expr, ($v1: expr, $v2: expr)) => {
        $apg.add_lambda_upsilon($e, $l, Value::Pair($v1, $v2))
    };

    ($apg: ident, $e: expr, $l: expr, $v: expr) => {
        $apg.add_lambda_upsilon($e, $l, $v)
    };
}

fn get_equalizer(h: &APGMorphism, _k: &APGMorphism) -> APG {
    h.check_source_labels();
    APG::default()
}

fn main() {
    let mut apg = APG::default();

    // set up
    add!(apg, "v1", "Person", ());
    add!(apg, "v2", "Person", ());
    add!(apg, "e1", "knows", (ev!(apg, "v1"), ev!(apg, "v2")));

    let mut apg1 = APG::default();
    let tp_right = Rc::new(Type::Lbl(Rc::new(Label("M".to_string()))));
    add!(apg1, "f1", "Sex", Value::Inl(Rc::new(Value::Unit), tp_right));
    let tp_left = Rc::new(Type::Lbl(Rc::new(Label("F".to_string()))));
    add!(apg1, "m1", "Sex", Value::Inr(tp_left, Rc::new(Value::Unit)));

    let mut apg2 = APG::default();
    let tp_right = Rc::new(Type::Lbl(Rc::new(Label("M".to_string()))));
    add!(apg2, "f1", "Sex", Value::Inl(Rc::new(Value::Unit), tp_right));
    let tp_left = Rc::new(Type::Lbl(Rc::new(Label("F".to_string()))));
    add!(apg2, "m1", "Sex", Value::Inr(tp_left, Rc::new(Value::Unit)));

    let apg1_ref = Rc::new(apg1);
    let apg2_ref = Rc::new(apg2);

    let mor1 = APGMorphism {
        from: apg1_ref.clone(),
        to: apg2_ref.clone(),
        element_mapping: |_e| Rc::new(Element("f1".to_string())),
    };

    let mor2 = APGMorphism {
        from: apg1_ref.clone(),
        to: apg2_ref.clone(),
        element_mapping: |e| e,
    };

    let equalizer = get_equalizer(&mor1, &mor2);
}
