open Batteries

let parse filename =
  let ch = open_in filename in
  Parser.program Lexer.token (Lexing.from_channel ch)

let () =
  let filename = Sys.argv.(1) in
  let p = parse filename in
  print_endline (Printer.debug_of_program p)

