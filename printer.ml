open Program

type debug_info = {
  indent: int;
  dummy: unit
}

let debug_of_ident _ ident =
  let Ident.Expr(name) = ident in name

let debug_of_dummy _ _ = "dummy"

let debug_of_type _ = function
  | Type.Int () -> "int"
  | Type.Bool () -> "bool"

let debug_of_params info params =
  "Params[" ^
  (List.fold_left (fun acc (name, typ) ->
       acc ^ debug_of_ident info name ^ ":" ^ debug_of_type info typ ^ ", "
     ) "" params) ^
  "]"

let debug_of_binop _ = function
  | Add -> "+"
  | Sub -> "-"
  | Mult -> "*"
  | Div -> "/"
  | Eq -> "="
  | Lt -> "<"
  | Gt -> ">"

let rec debug_of_expr info = function
  | Neg (e, ()) -> Printf.sprintf "Neg(%s)" (debug_of_expr info e)
  | BinOp (op, lhs, rhs, ()) ->
    Printf.sprintf "%s(%s, %s)" (debug_of_binop info op) (debug_of_expr info lhs) (debug_of_expr info rhs)
  | Block(block, ()) -> debug_of_block info block
  | App (f, args, ()) ->
    Printf.sprintf "App(%s, [%s])" (debug_of_ident info f) (debug_of_args info args)
  | If (cond, tr, fl, ()) ->
    Printf.sprintf "If(%s, %s, %s)"
      (debug_of_expr info cond)
      (debug_of_expr info tr)
      (debug_of_expr info fl)
  | Var (name, ()) -> debug_of_ident info name
  | IntLit (n, ()) -> string_of_int n
  | BoolLit (b, ()) -> string_of_bool b

and debug_of_args info = function
  | [] -> ""
  | x::xs ->
    Printf.sprintf "%s, %s" (debug_of_expr info x) (debug_of_args info xs)

and debug_of_statement info = function
  | Let(name, e) -> Printf.sprintf "Let(%s, %s)" (debug_of_ident info name) (debug_of_expr info e)
  | Expr e -> Printf.sprintf "Expr(%s)" (debug_of_expr info e)

and debug_of_block info (stmts, expr) =
  let indent = String.make info.indent ' ' in
  let info = { info with indent = info.indent+2 } in
  let inner_indent = String.make info.indent ' ' in
  "{\n" ^
  (List.fold_left (fun acc stmt ->
       acc ^ inner_indent ^ debug_of_statement info stmt ^ "\n"
    ) "" stmts) ^
  inner_indent ^ debug_of_expr info expr ^ "\n" ^
  indent ^ "}"

let debug_of_func info f =
  Printf.sprintf "Func(%s, %s, %s)" (debug_of_ident info f.name) (debug_of_params info f.params) (debug_of_block info f.body)

let debug_of_program funcs =
  let info = {
    indent = 0;
    dummy = ()
  } in
  "Program[\n" ^
  (List.fold_left (fun acc f ->
       acc ^ debug_of_func info f ^ "\n"
     ) "" funcs) ^
  "]"

