use druid::*;
use widget::*;

#[macro_export]
macro_rules! std_padding {
    ($child:expr) => {
        $child.padding(KeyOrValue::Concrete(Insets {
            x0: 30.0,
            y0: 20.0,
            x1: 30.0,
            y1: 20.0,
        }))
    };
}

pub(crate) use std_padding;

#[macro_export]
macro_rules! std_padding_less {
    ($child:expr) => {
        $child.padding(KeyOrValue::Concrete(Insets {
            x0: 30.0,
            y0: 5.0,
            x1: 30.0,
            y1: 5.0,
        }))
    };
}

pub(crate) use std_padding_less;

#[macro_export]
macro_rules! std_padding_less_text {
    ($child:expr) => {
        $child.padding(KeyOrValue::Concrete(Insets {
            x0: 30.0,
            y0: 5.0,
            x1: 120.0,
            y1: 5.0,
        }))
    };
}

pub(crate) use std_padding_less_text;
