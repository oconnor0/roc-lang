// Copyright (c) 2017, Matthew O'Connor

//! Abstract syntax tree for roc-lang
//!
//! Lua syntax specification;
//! from the Lua 5.3 Reference Manual http://www.lua.org/manual/5.3/manual.html
//!
//! Here is the complete syntax of Lua in extended BNF. As usual in extended
//! BNF, {A} means 0 or more As, and [A] means an optional A.
//! (For operator precedences, see §3.4.8; for a description of the terminals
//! Name, Numeral, and LiteralString, see §3.1.)

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Numeral;
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LiteralString;
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ellipsis;

/// chunk ::= block
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Chunk(Block);

/// block ::= {stat} [retstat]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Block(Vec<Stmt>, Option<ReturnStmt>);

/// stat ::=  ‘;’ |  varlist ‘=’ explist |  functioncall |  label |  break |
///           goto Name |  do block end |  while exp do block end |
///           repeat block until exp |
///           if exp then block {elseif exp then block} [else block] end |
///           for Name ‘=’ exp ‘,’ exp [‘,’ exp] do block end |
///           for namelist in explist do block end |
///           function funcname funcbody |
///           local function Name funcbody |  local namelist [‘=’ explist]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Stmt;

/// retstat ::= return [explist] [‘;’]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReturnStmt(ExprList);

/// label ::= ‘::’ Name ‘::’
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label(Name);

/// funcname ::= Name {‘.’ Name} [‘:’ Name]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FuncName(Vec<Name>, Option<Name>);

/// varlist ::= var {‘,’ var}
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VarList(Vec<Var>);

/// var ::=  Name | prefixexp ‘[’ exp ‘]’ | prefixexp ‘.’ Name
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Var {
  Name(Name),
  PrefixExpr(PrefixExpr, Expr),
  Expr(PrefixExpr, Name),
}

/// namelist ::= Name {‘,’ Name}
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NameList(Vec<Name>);

/// explist ::= exp {‘,’ exp}
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExprList(Vec<Expr>);

/// exp ::=  nil | false | true | Numeral | LiteralString | ‘...’ |
///          functiondef |  prefixexp | tableconstructor | exp binop exp |
///          unop exp
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expr {
  Nil,
  False,
  True,
  Numeral(Numeral),
  LiteralString(LiteralString),
  Ellipsis,
  FunctionDef,
  TableCtor(TableCtor),
  BinOp(Box<Expr>, Box<Expr>),
  UnOp(Box<Expr>),
}

/// prefixexp ::= var | functioncall | ‘(’ exp ‘)’
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrefixExpr;

/// functioncall ::=  prefixexp args | prefixexp ‘:’ Name args
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionCall;

/// args ::=  ‘(’ [explist] ‘)’ | tableconstructor | LiteralString
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Args;

/// functiondef ::= function funcbody
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionDef(FuncBody);

/// funcbody ::= ‘(’ [parlist] ‘)’ block end
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FuncBody(ParamList, Block);

/// parlist ::= namelist [‘,’ ‘...’] | ‘...’
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamList(NameList, Option<Ellipsis>);

/// tableconstructor ::= ‘{’ [fieldlist] ‘}’
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TableCtor(FieldList);

/// fieldlist ::= field {fieldsep field} [fieldsep]
/// fieldsep ::= ‘,’ | ‘;’
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldList(Vec<Field>);

/// field ::= ‘[’ exp ‘]’ ‘=’ exp | Name ‘=’ exp | exp
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Field {
  NameAssign(Name, Expr),
  ExprAssign(Expr, Expr),
  Expr(Expr),
}

/// binop ::=  ‘+’ | ‘-’ | ‘*’ | ‘/’ | ‘//’ | ‘^’ | ‘%’ |  ‘&’ | ‘~’ | ‘|’ |
///            ‘>>’ | ‘<<’ | ‘..’ |  ‘<’ | ‘<=’ | ‘>’ | ‘>=’ | ‘==’ | ‘~=’ |
///             and | or
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,
  IDiv,
  BitAnd,
  BitOr,
  BitXor,
  ShL,
  ShR,
  Concat,
  Equal,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  NotEqual,
  And,
  Or,
}

/// unop ::= ‘-’ | not | ‘#’ | ‘~’
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnOp {
  Negate,
  Not,
  Len,
  BitNot,
}

impl Name {
  pub fn new<'input>(name: &'input str) -> Name {
    Name {
      0: name.to_string(),
    }
  }
}