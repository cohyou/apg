pub mod element;
pub mod value;
pub mod label;
pub mod r#type;

use std::rc::Rc;
use std::fmt;
use std::collections::{HashMap, HashSet};

pub use element::Element;
pub use value::Value;
pub use label::Label;
pub use r#type::Type;


type Elements = HashSet<Rc<Element>>;
pub type Labels = HashSet<Rc<Label>>;
type LambdaUpsilon = HashMap<Rc<Element>, (Rc<Label>, Rc<Value>)>;

pub struct APG {
    pub name: String,
    pub elements: Elements,
    // values: HashSet<Rc<Value>>,    
    pub labels: Labels,
    pub lambda_upsilon: LambdaUpsilon,
}

impl fmt::Debug for APG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "[{}]", self.name);
        let _ = writeln!(f, "elements: {:?}", self.elements);
        // let _ = writeln!(f, "values:   {:?}", self.values);
        let _ = writeln!(f, "labels:   {:?}", self.labels);
        writeln!(f, "lambda_upsilon: {:?}", self.lambda_upsilon)
    }
}

pub fn product_of_apgs(apg1: &APG, apg2: &APG) -> APG {
    let new_labels1 = change_labels(&apg1.labels, &apg1.name);
    let new_labels2 = change_labels(&apg2.labels, &apg2.name);
    let mut new_labels = HashSet::new();
    for label1 in new_labels1.iter() {
        for label2 in new_labels2.iter() {
            let mut new_label_elem = vec![];
            new_label_elem.extend(label1.0.clone());
            new_label_elem.extend(label2.0.clone());
            new_labels.insert(Rc::new(Label(new_label_elem)));
        }
    }

    let new_elements1 = change_elements(&apg1.elements, &apg1.name);
    let new_elements2 = change_elements(&apg2.elements, &apg2.name);
    let mut new_elements = HashSet::new();
    for element1 in new_elements1.iter() {
        for element2 in new_elements2.iter() {
            let mut new_element_elem = vec![];
            new_element_elem.extend(element1.0.clone());
            new_element_elem.extend(element2.0.clone());
            new_elements.insert(Rc::new(Element(new_element_elem)));
        }
    }    

    let mut new_lambda_upsilon = change_lambda_upsilon(&apg1.lambda_upsilon, &apg1.name);
    new_lambda_upsilon.extend(change_lambda_upsilon(&apg2.lambda_upsilon, &apg2.name));

    let new_name = format!("{}*{}", apg1.name, apg2.name);
    APG::new(&new_name, new_elements, new_labels, new_lambda_upsilon)
}

fn change_labels(labels: &HashSet<Rc<Label>>, name: &str) -> HashSet<Rc<Label>> {
    if labels.contains(&Label(vec![])) {
        let mut res = HashSet::new();
        res.insert(Rc::new(Label(vec![vec![name.to_string()]])));
        res
    } else {
        labels.clone()
    }
}

fn change_elements(labels: &HashSet<Rc<Element>>, name: &str) -> HashSet<Rc<Element>> {
    if labels.contains(&Element(vec![])) {
        let mut res = HashSet::new();
        res.insert(Rc::new(Element(vec![vec![name.to_string()]])));
        res
    } else {
        labels.clone()
    }
}

fn change_lambda_upsilon(lambda_upsilon: &LambdaUpsilon, name: &str) -> LambdaUpsilon {
    if lambda_upsilon.len() == 1 && 
    lambda_upsilon[&Element::default()].0.as_ref() == &Label::default() {
        let mut res = HashMap::new();
        let new_element = Element(vec![vec![name.to_string()]]);
        let new_label = Label(vec![vec![name.to_string()]]);
        res.insert(Rc::new(new_element), (Rc::new(new_label), Rc::new(Value::Unit)));
        res
    } else {
        lambda_upsilon.clone()
    }
}

pub fn coproduct_of_apgs<F>(apg1: &APG, apg2: &APG) -> APG where F: Fn(&APG, &APG) -> String {
    let prefix1 = apg1.name.clone();
    let prefix2 = apg2.name.clone();

    let a = add_prefix_to_labels(&apg1.labels, &prefix1);
    let b = add_prefix_to_labels(&apg2.labels, &prefix2);
    let labels = &a | &b;

    let a_elements = add_prefix_to_elements(&apg1.elements, &prefix1);
    let b_elements = add_prefix_to_elements(&apg2.elements, &prefix2);
    let elements = &a_elements | &b_elements;

    let mut a_f = replace_value(&apg1.lambda_upsilon, &prefix1);
    let b_f = replace_value(&apg2.lambda_upsilon, &prefix2);

    a_f.extend(b_f);

    let new_name = prefix1.clone() + &prefix2;
    APG::new(&new_name, elements, labels, a_f)
}

fn add_prefix_to_labels(labels: &HashSet<Rc<Label>>, prefix: &str) -> HashSet<Rc<Label>> {
    let new_labels = labels.clone();
    labels.iter()
        .map(|label| {
            let 
            Rc::new(Label(prefix.to_string() + &e.as_ref().0))
        })
        .collect()
}

fn add_prefix_to_elements(elements: &HashSet<Rc<Element>>, _prefix: &str) -> HashSet<Rc<Element>> {
    elements.iter().cloned().collect()
        // .map(|e| {
        //     let Element(e_name) = e.as_ref();
        //     Rc::new(Element(prefix.to_string() + e_name))    
        // }
        // .collect()
}

fn replace_value(lambda_upsilon: &HashMap<String, (String, Rc<Value>)>, prefix: &str)
-> HashMap<String, (String, Rc<Value>)> {
    lambda_upsilon.iter()
        // .inspect(|(k, v)| println!("replace_value: {:?} {:?}", k, v))
        .map(|(k, v)| {
            let new_key_sym = prefix.to_string() + k;
            let new_label_sym = prefix.to_string() + v.0.as_ref();
            match v.1.as_ref() {
                Value::Id(rc_element) => {
                    if let Element(e_name) = rc_element.as_ref() {
                        let new_sym = prefix.to_string() + e_name;
                        (new_key_sym, (new_label_sym, Rc::new(Value::Id(Rc::new(Element(new_sym))))))
                    } else {
                        unimplemented!()
                    }
                },
                _ => (new_key_sym, (new_label_sym, v.1.clone())),
            }
        })
        .collect::<HashMap<String, (String, Rc<Value>)>>()
}

pub fn make_named_one(name: &str) -> APG {
    let mut elements = HashSet::new();
    let e = Rc::new(Element::default());
    elements.insert(e.clone());
    let mut labels = HashSet::new();
    let l = Rc::new(Label::default());
    labels.insert(l.clone());
    let mut lambda_upsilon1 = HashMap::new();
    lambda_upsilon1.insert(e.clone(), (l.clone(), Rc::new(Value::Unit)));
    APG::new(name, elements, labels, lambda_upsilon1)
}

#[allow(dead_code)]
impl APG {
    pub fn new(name: &str, elements: Elements, labels: Labels, lambda_upsilon: LambdaUpsilon) -> APG {
        APG {
            name: name.to_string(),
            elements: elements,
            labels: labels,
            lambda_upsilon: lambda_upsilon,
        }
    }
    fn zero() -> APG {
        APG::new("", HashSet::default(), HashSet::default(), HashMap::default())
    }

    // pub fn add_element(&mut self, name: &str) {
    //     let element = Element(name.to_string());
    //     self.elements.insert(Rc::new(element));
    // }

    // fn get_element(&self, name: &str) -> Option<Rc<Element>> {
    //     for element in self.elements.iter() {       
    //         if let Element(e_name) = element.as_ref() {
    //             if e_name == name {
    //                 return Some(element.clone());
    //             } 
    //         }   
    //     }

    //     None
    // }

    // pub fn add_value(&mut self, v: Value) {
    //     let v = Rc::new(v);
    //     self.values.insert(v.clone());
    // }
    
    // fn add_element_value(&mut self, element_name: &str) {
    //     let v = Value::Id(self.get_element(element_name).unwrap());
    //     // self.add_value(v);
    // }

    // pub fn get_element_value(&self, element_name: &str) -> Option<Rc<Value>> {
    //     for value in self.values.iter() {            
    //         if let Value::Id(elem) = value.as_ref() {    
    //             if elem.0 == element_name {
    //                 return Some(value.clone());
    //             }
    //         }            
    //     }

    //     None        
    // }

    // fn add_pair_value(&mut self, v1: Rc<Value>, v2: Rc<Value>) {
    //     self.add_value(Value::Pair(v1, v2));
    // }

    // pub fn add_label(&mut self, name: &str) {
    //     let lbl = Label(name.to_string());
    //     self.labels.insert(Rc::new(lbl));
    // }

    // fn get_label(&self, name: &str) -> Option<Rc<Label>> {
    //     for label in self.labels.iter() {            
    //         if label.0 == name {
    //             return Some(label.clone());
    //         }            
    //     }

    //     None
    // }

    // pub fn add_lambda_upsilon(&mut self, e: &str, l: &str, v: Value) {
    //     self.add_element(e);
    //     self.add_element_value(e);
    //     self.add_label(l);
    //     let v_rc = Rc::new(v);
    //     // self.values.insert(v_rc.clone());
    //     self.lambda_upsilon.insert(e.to_string(), (l.to_string(), v_rc.clone()));
    // }

    // fn filter_labels_by_element<P>(&self, pred: P) -> HashSet<Rc<Label>>
    //     where P: for<'r> FnMut(&'r Rc<Element>) -> bool
    // {
    //     self.elements.iter().cloned()
    //         .filter(pred)
    //         .map(|e| {
    //             if let Element(e_name) = e.as_ref() {
    //                 Rc::new(Label(self.lambda_upsilon[e_name].0.clone()))
    //             } else {
    //                 unimplemented!()
    //             }
    //         })
    //         .collect()
    // }

    // fn filter_values<P>(&self, pred: P) -> HashSet<Rc<Value>>
    //     where P: for<'r> FnMut(&'r Rc<Value>) -> bool
    // {
    //     self.values.iter().cloned().filter(pred).collect()
    // }

    fn filter_elements<P>(&self, pred: P) -> HashSet<Rc<Element>>
        where P: for<'r> FnMut(&'r Rc<Element>) -> bool
    {
        self.elements.iter().cloned().filter(pred).collect()
    }
}

impl Default for APG {
    fn default() -> APG {
        APG {
            name: "".to_string(),
            elements: HashSet::default(),
            // values: HashSet::default(), 
            labels: HashSet::default(),
            lambda_upsilon: HashMap::default(),
        }
    }
}

type ElemMapping = fn(Rc<Element>) -> Rc<Element>;

#[allow(dead_code)]
pub struct APGMorphism {
    from: Rc<APG>,
    to: Rc<APG>,
    elem_mapping: ElemMapping,
}

impl APGMorphism {
    pub fn new(from: Rc<APG>, to: Rc<APG>, mapping: ElemMapping) -> Self {
        APGMorphism {
            from: from,
            to: to,
            elem_mapping: mapping,
        }
    }
}

// pub fn get_equalizer(h: &APGMorphism, k: &APGMorphism) -> APG {
//     APG {
//         name: "".to_string(),
//         elements: h.from.filter_elements(|e| {
//             (h.elem_mapping)(e.clone()) == (k.elem_mapping)(e.clone())
//         }),
//         // values: h.from.filter_values(|_a| true),
//         labels: h.from.filter_labels_by_element(|e| {
//             (h.elem_mapping)(e.clone()) == (k.elem_mapping)(e.clone())
//         }),
//         lambda_upsilon: HashMap::default(),
//     }
// }
