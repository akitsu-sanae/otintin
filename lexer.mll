{
open Parser

exception LexError of string
}

let space = [' ' '\t' '\n' '\r']
let non_return = [^'\n']
let digit = ['0'-'9']
let alpha = ['A'-'Z' 'a'-'z' '_']
let alnum = digit | alpha | '\''

rule token = parse
  | space+ { token lexbuf }
  | "/*" {
      comment lexbuf;
      token lexbuf
    }
  | digit+ { INT (Lexing.lexeme lexbuf |> int_of_string) }
  | '+' { PLUS }
  | '-' { MINUS }
  | '*' { STAR }
  | '/' { SLASH }
  | '=' { EQUAL }
  | '<' { LESS }
  | '>' { GREATER }
  | ';' { SEMICOLON }
  | ':' { COLON }
  | ',' { COMMA }
  | '(' { LPAREN }
  | ')' { RPAREN }
  | '{' { LBRACE }
  | '}' { RBRACE }
  | "true" { TRUE }
  | "false" { FALSE }
  | "func" { FUNC }
  | "let" { LET }
  | "if" { IF }
  | "else" { ELSE }
  | "int" { TINT }
  | "bool" { TBOOL }
  | alpha alnum* { IDENT (Ident.Expr (Lexing.lexeme lexbuf)) }
  | eof { EOF }
  | _ {
      let msg =
        Printf.sprintf
          "unknown token %s near characters %d-%d"
          (Lexing.lexeme lexbuf)
          (Lexing.lexeme_start lexbuf)
          (Lexing.lexeme_end lexbuf)
      in
      raise @@ LexError msg
    }
and comment = parse
  | "*/" { () }
  | "/*" {
      comment lexbuf;
      comment lexbuf
    }
  | _ {
      comment lexbuf
    }



