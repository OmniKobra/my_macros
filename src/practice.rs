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
    ($ident: ident, $ty:ty, $init:expr) => {
        let mut $ident: $ty = $init;
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
            0 => "Zero",
            x if x > 0 => "Positive",
            _ => "Negative",
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

#[macro_export]
macro_rules! repeat {
    ($lit:literal,$n:expr) => {
        for _ in 1..=$n {
            println!("{}", $lit);
        }
    };
}

#[macro_export]
macro_rules! make_vec {
    ($($expr:expr),*) => {
        vec![
            $(
                $expr
            ),*
        ]
    };
}

#[macro_export]
macro_rules! dbg_var {
    ($ident:ident) => {
        println(
            "Variable name: {}, Variable Value: {}",
            stringify!($ident),
            $ident,
        )
    };
}

#[macro_export]
macro_rules! count_tuples {
    ($(($($ident:ident),+)),+) => {
        {
        let mut counter = 0;
        $(
            counter += 1;
        ),+
        counter
    }
    };
}

#[macro_export]
macro_rules! newtype {
    ($ident:ident, $ty:ty) => {
        struct $ident($ty);
    };
}

#[macro_export]
macro_rules! enum_builder {
    ($ident:ident: $($variant:ident),*) => {
        enum $ident {
            $(
                $variant
            ),*
        }
    };
}

// expands the token tree (code) only if in debug mode
#[macro_export]
macro_rules! if_dbg {
    ($tt:tt) => {
        // cfg! macro evaluates to bool, debug_assertions built-in flag active in debug mode
        if cfg!(debug_assertions) {
            $tt
        }
    };
}

#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr),*) => {
        {
        use std::collections::HashMap;
        let mut _map = HashMap::new();
        (
            _map.insert($key, $val);
        )*
        _map
        }
    };
}

#[macro_export]
macro_rules! chain_print {
    ($($lit:literal),+) => {
        println!("{}",concat!($($lit),+))
    };
}

#[macro_export]
macro_rules! alias {
    ($ident:ident = $ty:ty) => {
        type $ident = $ty;
    };
}

#[macro_export]
macro_rules! getter {
    ($($vis:vis $ident:ident { $($field:ident : $ty:ty),+ }),+) => {
        $(
           $vis struct $ident {
                $($field: $ty),+
            }
            impl $ident {
                $(
                   pub fn $field(&self) -> &$ty {
                        &self.$field
                    }
                )+
            }
        )+
    };
}

#[macro_export]
macro_rules! for_range {
    ($ident:ident in $lit1:literal..$lit2:literal => $block:block) => {
        for $ident in $lit1..$lit2 {
            $block
        }
    };
}

#[macro_export]
macro_rules! match_bool {
    ($expr:expr, $true:expr, $false:expr) => {
        match $expr {
            true => $true,
            false => $false,
        }
    };
}

#[macro_export]
macro_rules! lazy_val {
    ($ident:ident, $ty:ty, $expr:expr) => {
        use std::cell::LazyCell;
        static $ident: LazyCell = LazyCell::<$ty>::new(|| $expr);
    };
}

pub struct Query<T1, T2, T3> {
    pub select: T1,
    pub from: T2,
    pub where_clause: T3,
}

#[macro_export]
macro_rules! query {
    (select: $x:expr, from: $y:expr, where: $cond:expr) => {{
        Query {
            select: $x,
            from: $y,
            where_clause: $cond,
        }
    }};
}

/*
let q = query!(select: "name", from: "users", where: "age > 18");

println!("SELECT {} FROM {} WHERE {}", q.select, q.from, q.where_clause);
*/
