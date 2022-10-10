use zhoo::back::codegen;
use zhoo::front::parser;

fn main() {
  let filepath = "samples/simple.zo";
  let source = std::fs::read_to_string(filepath).expect("to read file");
  let program = parser::parse(&source);

  println!("\n{:?}", program);

  let codegen = codegen::cranelift::aot::generate(&program);

  match codegen.build(false) {
    Ok(done) => {
      done();
    }
    Err(error) => {
      eprintln!("{error}");
      eprintln!("bad bad bad");
    }
  }
}
