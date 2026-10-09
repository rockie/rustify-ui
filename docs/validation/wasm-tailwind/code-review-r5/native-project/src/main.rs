pub use rustify_style::{compiler,text};
mod old_scope;
fn main(){
 let input=std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
 let cases:serde_json::Value=serde_json::from_str(&input).unwrap();
 let mut rows=Vec::new();
 for case in cases.as_array().unwrap(){
  let css=case["css"].as_str().unwrap();
  let ast=compiler::parser::parse(css,32).unwrap();
  let output=|nodes:Result<Vec<compiler::ast::AstNode>,compiler::CompileError>|match nodes {Ok(nodes)=>serde_json::json!({"css":compiler::ast::to_css(&nodes).to_utf8().unwrap()}),Err(e)=>serde_json::json!({"error":format!("{e:?}")})};
  rows.push(serde_json::json!({"id":case["id"],"input":css,"before_r4":output(old_scope::stylesheet(&ast,"review-r4",32)),"after_r5":output(rustify_style::scope::stylesheet(&ast,"review-r4",32))}));
 }
 println!("{}",serde_json::to_string_pretty(&rows).unwrap());
}
