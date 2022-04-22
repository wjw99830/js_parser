use crate::parser::parse;

mod context;
mod node;
mod parser;
mod string;
mod token;

// const SRC: &str = "
// const a = getNumber() ?? 1;
// const b = 2;
// const str = \"Hello, World!\";
// const reg = /?!(maybe)/i;

// class Parent {
//   #name = 'parent';
// }

// class child extends Parent {
//   #name = 'child';
// }

// function plus(a, b) {
//   return a + b;
// }

// const minus = (a, b) => a - b;

// plus(a, b);
// minus(a, b);
// ";

const SRC: &str = "
function plus(a, b) {
  return a + b
}
";

fn main() {
    parse(SRC);
}
