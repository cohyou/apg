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
