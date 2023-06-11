type 'a ast = Ast of 'a

module type Interp = sig
  type 'a repr

  val ast : 'a repr -> 'a ast repr
  val bool : bool -> bool repr
  val int : int -> int repr
  val add : int repr -> int repr -> int repr
  val if_stmt : bool repr -> 'a repr -> 'a repr -> 'a repr
end

module Eval = struct
  type 'a repr = 'a

  let ast x = Ast x
  let bool b = b
  let int i = i
  let add = ( + )
  let if_stmt c a b = if c then a else b
  let unwrap (x : 'a repr) : 'a = x
end

module Display = struct
  type 'a repr = string

  let ast x = x
  let bool = string_of_bool
  let int = string_of_int
  let add a b = "(" ^ a ^ ") + (" ^ b ^ ")"
  let if_stmt c a b = "if (" ^ c ^ ") then { " ^ a ^ " } else { " ^ b ^ " }"
end

let rec ident idx =
  if idx = 0 then ""
  else
    let rest, digit = (idx / 26, idx mod 26) in
    let c = Char.escaped (Char.chr (96 + digit)) in
    c ^ ident rest

module CCodegen = struct
  type 'a repr = int -> string * string

  let ast x i =
    let ast, id = x i in
    ( "#include <stdbool.h>\n#include <stdio.h>\nint main() {\n" ^ ast
      ^ "printf(\"%d\\n\", " ^ id ^ ");\nreturn 0;\n}\n",
      id )

  let bool b i =
    let id = ident i in
    ("bool " ^ id ^ " = " ^ string_of_bool b ^ ";\n", id)

  let int a i =
    let id = ident i in
    ("int " ^ id ^ " = " ^ string_of_int a ^ ";\n", id)

  let add a b i =
    let id, (a, a_id), (b, b_id) = (ident i, a (2 * i), b (3 * i)) in
    (a ^ b ^ "int " ^ id ^ " = " ^ a_id ^ " + " ^ b_id ^ ";\n", id)

  let if_stmt c a b i =
    let id, (c, c_id), (a, a_id), (b, b_id) =
      (ident i, c (5 * i), a (7 * i), b (11 * i))
    in
    ( c ^ "int " ^ id ^ ";\nif (" ^ c_id ^ ") {\n" ^ a ^ id ^ " = " ^ a_id
      ^ ";\n} else {\n" ^ b ^ id ^ " = " ^ b_id ^ ";\n}\n",
      id )

  let unwrap x =
    let code, _ = x 1 in
    code
end

module MyAst (I : Interp) = struct
  open I

  let my_ast =
    ast
      (if_stmt
         (if_stmt (bool false) (bool false) (bool true))
         (add (int 10) (int 1))
         (int (-1)))
end

module EvalAst = MyAst (Eval)
module DisplayAst = MyAst (Display)

let (Ast result_eval) = EvalAst.my_ast |> Eval.unwrap
let result_disp = DisplayAst.my_ast
let _ = Printf.printf "//%d\n//%s\n" result_eval result_disp

module CAst = MyAst (CCodegen)
let _ = print_string (CAst.my_ast |> CCodegen.unwrap)