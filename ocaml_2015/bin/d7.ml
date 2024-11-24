open Ocaml_2015.Lib

let filename = "bin/input/d7.txt"

type op = 
  | And of (string * string)
  | Or of (string * string)
  | LShift of (string * int)
  | RShift of (string * int)
  | Not of string
  | Assign of string
  | Val of int

let target_of_string str =
  let len = String.length str in
  let raw_target = String.sub str 1 (len - 1) in
  String.trim raw_target

let op_and_args_of_string str =
  let op_and_args = String.split_on_char ' ' str in
  match List.length op_and_args with
  | 1 -> (
    let val_opt = List.nth op_and_args 0 in
    let create_val x = Val x in
    try int_of_string val_opt |> create_val
    with Failure _ -> Assign val_opt
  )
  | 2 -> Not (List.nth op_and_args 1)
  | 3 -> (
    let op_str = (List.nth op_and_args 1) in
    match op_str  with
    | _ when String.starts_with ~prefix:"AND" op_str -> 
      let x = List.nth op_and_args 0 in
      let y = List.nth op_and_args 2 in
      And (x, y)
    | _ when String.starts_with ~prefix:"OR" op_str -> 
      let x = List.nth op_and_args 0 in
      let y = List.nth op_and_args 2 in
      Or (x, y)
    | _ when String.starts_with ~prefix:"LSHIFT" op_str -> 
      let x = List.nth op_and_args 0 in
      let y = int_of_string (List.nth op_and_args 2) in
      LShift (x, y)
    | _ when String.starts_with ~prefix:"RSHIFT" op_str -> 
      let x = List.nth op_and_args 0 in
      let y = int_of_string (List.nth op_and_args 2) in
      RShift (x, y)
    | str -> 
      failwith ("Unexpected operator: " ^ str)
  )
  | _ -> failwith "Unable to parse op and args"


let setup lines =
  let hash = Hashtbl.create 1000 in
  let lines_len = List.length lines in
  for i = 0 to (lines_len - 1) do
    let parts = String.split_on_char '-' (List.nth lines i) in
    let op_and_args = String.trim (List.nth parts 0) in
    let op_and_args = op_and_args_of_string op_and_args in
    let target = String.trim (List.nth parts 1) in
    let target = target_of_string target in
    Hashtbl.add hash target op_and_args
  done;
  hash

let rec traverse hash next_target =
  try int_of_string next_target
  with Failure _ ->
  (* Override b to answer from p1 *)
  (* if next_target = "b" then 956 else *)
  let op_and_args = Hashtbl.find hash next_target in
  match op_and_args with
  | Val x -> x
  | Assign x ->
    let res = traverse hash x in
    let new_op = Val res in
    Hashtbl.replace hash next_target new_op;
    res
  | Or (x, y) ->
    let x = traverse hash x in
    let y = traverse hash y in
    let res = Int.logor x y in
    let new_op = Val res in
    Hashtbl.replace hash next_target new_op;
    res
  | And (x, y) ->
    let x = traverse hash x in
    let y = traverse hash y in
    let res = Int.logand x y in
    let new_op = Val res in
    Hashtbl.replace hash next_target new_op;
    res
  | Not x ->
    let x = traverse hash x in
    let res = 65535 - x in
    let new_op = Val res in
    Hashtbl.replace hash next_target new_op;
    res
  | LShift (x, y) ->
    let x = traverse hash x in
    let res = Int.shift_left x y in
    let new_op = Val res in
    Hashtbl.replace hash next_target new_op;
    res
  | RShift (x, y) ->
    let x = traverse hash x in
    let res = Int.shift_right x y in
    let new_op = Val res in
    Hashtbl.replace hash next_target new_op;
    res

let p1 filename start =
  let lines = read_lines filename in
  let hash = setup lines in
  traverse hash start

let () =
  let p1_res = p1 filename "a" in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
