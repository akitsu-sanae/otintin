%{
open Program

exception ParseError of string
%}

%token <Ident.expr> IDENT
%token <int> INT
%token PLUS MINUS STAR SLASH
%token EQUAL LESS GREATER
%token SEMICOLON COLON COMMA
%token LPAREN RPAREN
%token LBRACE RBRACE

%token TRUE FALSE
%token FUNC
%token LET
%token IF ELSE

%token TINT TBOOL

%token EOF

%left EQUAL GREATER LESS
%left PLUS MINUS
%left STAR SLASH

%start <Program.program> program

%%

program:
  | fs = list(func) EOF { fs}
  | error {
    let pos = Parsing.symbol_start_pos () in
    let msg =
      Printf.sprintf "parse error near at (%d:%d)"
      pos.pos_lnum pos.pos_cnum
    in
    raise @@ ParseError msg
  }

func:
  FUNC id=IDENT p=params b=block {
    {
      name = id;
      params = p;
      body = b
    }
  }

param: name=IDENT COLON t=typ { (name, t) }
params: ps = list(param) { ps }

block:
  LBRACE bb=block_body RBRACE { bb }

statement:
  | LET name=IDENT EQUAL e=expr SEMICOLON { Let(name, e) }
  | e=expr SEMICOLON { Expr(e) }

block_body:
  | e=expr { ([], e) }
  | s=statement bb=block_body {
    let ss, e = bb in
    (s::ss, e)
  }

expr:
  | e=primitive_expr { e }
  | MINUS e=primitive_expr { Neg(e, ()) }
  | lhs=expr PLUS rhs=expr { BinOp(Add, lhs, rhs, ()) }
  | lhs=expr MINUS rhs=expr { BinOp(Sub, lhs, rhs, ()) }
  | lhs=expr STAR rhs=expr { BinOp(Mult, lhs, rhs, ()) }
  | lhs=expr SLASH rhs=expr { BinOp(Div, lhs, rhs, ()) }
  | lhs=expr EQUAL rhs=expr { BinOp(Eq, lhs, rhs, ()) }
  | lhs=expr LESS rhs=expr { BinOp(Lt, lhs, rhs, ()) }
  | lhs=expr GREATER rhs=expr { BinOp(Gt, lhs, rhs, ()) }
  | f=IDENT LPAREN args=args RPAREN { App(f, args, ()) }
  | IF cond=expr tr=block ELSE fl=block { If(cond, Block (tr, ()), Block(fl, ()), ()) }

primitive_expr:
  | name=IDENT { Var(name, ()) }
  | n=INT { IntLit(n, ()) }
  | TRUE { BoolLit (true, ()) }
  | FALSE { BoolLit (false, ()) }
  | LPAREN e=expr RPAREN { e }
  | e=block { Block(e, ()) }

args:
  args=separated_list(COMMA, expr) { args }

typ:
  | TINT { Type.Int () }
  | TBOOL { Type.Bool () }

