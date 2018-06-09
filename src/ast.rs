#[derive(PartialEq, Debug, Clone)]
pub enum AST {
  PtrInc,
  PtrDec,
  ValInc,
  ValDec,
  Out,
  In,
  Loop(Vec<AST>),
}
