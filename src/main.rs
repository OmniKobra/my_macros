/**
*
* macro_rules! $name {
   $rule0 ;
   $rule1 ;
   // …
   $ruleN ;
 }        capture
* Rule: ($matcher) => {$expansion}

?: zero or one
*: zero or many
+: one or many

*
$arg (capture) types:
* block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
* expr: an expression
* ident: an identifier (this includes keywords)
* item: an item, like a function, struct, module, impl, etc.
* lifetime: a lifetime (e.g. 'foo, 'static, ...)
* literal: a literal (e.g. "Hello World!", 3.14, '🦀', ...)
* meta: a meta item; the things that go inside the #[...] and #![...] attributes
* pat: a pattern
* path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
* stmt: a statement
* tt: a single token tree
* ty: a type
* vis: a possible empty visibility qualifier (e.g. pub, pub(in crate), ...)

*/
#[macro_use]
extern crate my_macros;
use my_macros::Foo;

fn main() {
    println!("Hello, world!");
    let foo = Foo::new();
    ex!();
    let square_1 = square!(1);
    let least = min!(5; 3, 4, 7, 19, 2);
    make_var!(foo);
    println!("{}", foo);
    let o: make_option!(i32) = Some(8);
    let sum = sum!(1, 2, 3);
    println!("{}", sum);
}
