
type info = ()

type binop =
  | Add | Sub | Mult | Div
  | Eq | Lt | Gt

type expr =
  | Neg of expr * info
  | BinOp of binop * expr * expr * info
  | Block of block * info
  | App of Ident.expr * expr list * info
  | If of expr * expr * expr * info
  | Var of Ident.expr * info
  | IntLit of int * info
  | BoolLit of bool * info
and block = statement list * expr
and statement =
  | Let of Ident.expr * expr
  | Expr of expr

type func = {
  name: Ident.expr;
  params: (Ident.expr * Type.typ) list;
  body: block
}

type program = func list


