open Ocaml_2024.Lib

let filename = "input/d7.txt"

let concat_ints x y =
  let x = string_of_int x in
  let y = string_of_int y in
  let concat = x ^ y in
  int_of_string concat

let add x y = x + y

let mult x y = x * y

let parse_line line =
  let parts = String.split_on_char ':' line in
  let parts = List.map (fun s -> String.trim s) parts in
  let target = int_of_string (List.nth parts 0) in
  let factors = String.split_on_char ' ' (List.nth parts 1) in
  let factors = List.map (fun s -> String.trim s) factors in
  let factors = List.map (fun s -> int_of_string s) factors in
  (target, factors)

let can_make_target target factors ops =
  let rec next rest curr =
    if curr > target then 
      false 
    else
      match rest with
      | [] -> curr == target
      | factor::rest ->
        List.fold_left (fun acc op ->
          if acc then true else
          let try_res = op curr factor in
          next rest try_res
        ) false ops
  in
  next factors 0

let p1 lines =
  List.fold_left (fun acc line ->
    let (target, factors) = parse_line line in
    if can_make_target target factors [add; mult] then acc + target else acc
  ) 0 lines

let p2 lines =
  List.fold_left (fun acc line ->
    let (target, factors) = parse_line line in
    if can_make_target target factors [add; mult; concat_ints] then 
      acc + target 
    else acc
  ) 0 lines

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
