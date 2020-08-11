mod apg;

pub use apg::{APG, APGMorphism, get_equalizer};
pub use apg::{Element, Value, Label, Type};

#[macro_export]
macro_rules! ev {
    ($apg:ident, $e:ident) => {
        { $apg.get_element_value(stringify!($e)).unwrap() }
    };
}

#[macro_export]
macro_rules! add {
    ($apg:ident[$e:ident: $l:ident]) => {
        $apg.add_lambda_upsilon(stringify!($e), stringify!($l), Value::Unit)
    };
    ($apg:ident[$e:ident: $l:ident <$v1:ident * $v2:ident>]) => {
        $apg.add_lambda_upsilon(stringify!($e), stringify!($l), Value::Pair($v1, $v2))
    };
    ($apg:ident[$e:ident: $l:ident <$v1:ident L+ $v2:ident>]) => {
        $apg.add_lambda_upsilon(stringify!($e), stringify!($l), inl!($v1, $v2))
    };
    ($apg:ident[$e:ident: $l:ident <$v1:ident R+ $v2:ident>]) => {
        $apg.add_lambda_upsilon(stringify!($e), stringify!($l), inr!($v1, $v2))
    };
    ($apg:ident[$e:ident: $l:ident - $v:expr]) => {
        $apg.add_lambda_upsilon(stringify!($e), stringify!($l), $v)
    };
}

#[macro_export]
macro_rules! label_type {
    ($n: ident) => {{
        let label_m = Rc::new(Label(stringify!($n).to_string()));
        Rc::new(Type::Lbl(label_m.clone()))
    }};
}

#[macro_export]
macro_rules! mor {
    ($from: ident, $to: ident, $mapping: expr) => {
        APGMorphism::new($from.clone(), $to.clone(), $mapping)
    };
}

#[macro_export]
macro_rules! inl {
    ($val: ident, $type: ident) => {
        Value::Inl($val.clone(), $type.clone())
    };
}

#[macro_export]
macro_rules! inr {
    ($type: ident, $val: ident) => {
        Value::Inr($type.clone(), $val.clone())
    };
}

#[macro_export]
macro_rules! elem {
    ($n:ident) => {
        Rc::new(Element::E(stringify!($n).to_string()))
    };
}
