
let rec eval_expr fs = ()

let eval program =
  let main = List.find (fun f -> f.name = "main") program in
  eval_expr program Block(main.body)



