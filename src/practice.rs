#[macro_export]
macro_rules! square {
    ($lit:literal) => {
        $lit * $lit
    };
}

#[macro_export]
macro_rules! min {
    ($arg1:expr; $( $arg2:expr ),+ ) => {
        {
            let mut to_return = $arg1;
            $(
                if $arg2 < to_return {
                    to_return = $arg2;
                }
            )+
            to_return
        }
    };
}

#[macro_export]
macro_rules! make_var {
    ($ident: ident) => {
        let mut $ident: i32 = 0;
    };
}

#[macro_export]
macro_rules! make_option {
    ($ty:ty) => {
        Option<$ty>
    };
}

#[macro_export]
macro_rules! match_num {
    ($expr:expr) => {
        match $expr {
            0 => "Zero".into(),
            x if x > 0 => "Positive".into(),
            _ => "Negative".into(),
        }
    };
}

#[macro_export]
macro_rules! sum {
    ($($expr:expr),+) => {{
        let mut sum = 0;
        $(
            sum += $expr;
        )+
        sum
    };
}}

#[macro_export]
macro_rules! make_struct {
    ($name:ident { $($field:ident, $ty:ty),+ }) => {
        struct $name {
            $( $field: $ty),+
        }
    };
}

#[macro_export]
macro_rules! make_fn {
    ($(vis:vis)? $name:ident($($arg:ident : $typ:ty),*) -> $ty:ty $block: block) => {
       $( $vis )? fn $name($($arg: $typ),*) -> $ty $block
    };
}

#[macro_export]
macro_rules! debug_print {
    ($expr:expr, $($lit:literal)?) => {
        println!("{} {}", $expr, $($lit)?);
    };
}

#[macro_export]
macro_rules! matrix {
    ($($($expr:expr),+);+) => {
        {
        let mut outer = Vec::new();
      $(
       outer.push( vec! [$(
            $expr
        ),+]);
       )+
        outer
    }};
}
