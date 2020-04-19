use std::rc::Rc;
use std::collections::{HashMap, HashSet};
// use std::cell::RefCell;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Element(String);

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{:?}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Value {
    Unit,
    Inl(Rc<Value>, Rc<Type>),
    Inr(Rc<Type>, Rc<Value>),
    Pair(Rc<Value>, Rc<Value>),
    Prim(Rc<Value>),
    Id(Rc<Element>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Inl(v, t) => write!(f, "{:?} + {:?}", v, t),
            Value::Inr(t, v) => write!(f, "{:?} + {:?}", t, v),
            Value::Pair(v1, v2) => write!(f, "({:?}, {:?})", v1, v2),
            Value::Prim(v) => write!(f, "P{:?}", v),
            Value::Id(e) => write!(f, "{:?}", e),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Label(String);

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{:?}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Type {
    Zero,
    One,
    Sum(Rc<Type>, Rc<Type>),
    Product(Rc<Type>, Rc<Type>),
    Prim,
    Lbl(Rc<Label>),
}

// #[derive(Debug)]
struct APG {
    elements: HashSet<Rc<Element>>,
    values: HashSet<Rc<Value>>,    
    labels: HashSet<Rc<Label>>,
    lambda_upsilon: HashMap<String, (String, Rc<Value>)>,
}

impl fmt::Debug for APG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "elements: {:?}", self.elements);
        writeln!(f, "values:   {:?}", self.values);
        writeln!(f, "labels:   {:?}", self.labels);
        writeln!(f, "lambda_upsilon: {:?}", self.lambda_upsilon)
    }
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
            if let Value::Id(elem) = value.as_ref() {    
                if elem.0 == element_name {
                    return Some(value.clone());
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
        self.add_element_value(e);
        self.add_label(l);
        let v_rc = Rc::new(v);
        self.values.insert(v_rc.clone());
        self.lambda_upsilon.insert(e.to_string(), (l.to_string(), v_rc.clone()));
    }

    fn filter_labels_by_element<P>(&self, pred: P) -> HashSet<Rc<Label>>
        where P: for<'r> FnMut(&'r Rc<Element>) -> bool
    {
        self.elements.iter().cloned()
            .filter(pred)
            .map(|e| Rc::new(Label(self.lambda_upsilon[&e.as_ref().0].0.clone())))
            .collect()
    }

    fn filter_values<P>(&self, pred: P) -> HashSet<Rc<Value>>
        where P: for<'r> FnMut(&'r Rc<Value>) -> bool
    {
        self.values.iter().cloned().filter(pred).collect()
    }

    fn filter_elements<P>(&self, pred: P) -> HashSet<Rc<Element>>
        where P: for<'r> FnMut(&'r Rc<Element>) -> bool
    {
        self.elements.iter().cloned().filter(pred).collect()
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

// impl APGMorphism {
//     fn check_source_labels(&self) {
//         self.from.filter_labels(|_a| true);
//         self.from.filter_elements(|_a| true);
//     }
// }

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

fn get_equalizer(h: &APGMorphism, k: &APGMorphism) -> APG {
    APG {
        elements: h.from.filter_elements(|e| (h.element_mapping)(e.clone()) == (k.element_mapping)(e.clone())),
        values: h.from.filter_values(|_a| true),
        labels: h.from.filter_labels_by_element(|e| (h.element_mapping)(e.clone()) == (k.element_mapping)(e.clone()) ),
        lambda_upsilon: HashMap::default(),
    }
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
    println!("<FROM>\n{:?}", apg1_ref);
    println!("<TO>\n{:?}", apg2_ref);
    println!("<EQ>\n{:?}", equalizer);
}
