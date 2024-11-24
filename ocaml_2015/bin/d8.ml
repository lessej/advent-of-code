open Ocaml_2015.Lib

let filename = "bin/input/d8.txt"

let rec count_in_line i curr_total line =
  let line_len = String.length line in
  if i = 0 || i = (line_len - 1) then count_in_line (i + 1) curr_total line else 
  if i >= line_len then curr_total else
  let i = match line.[i] with
  | '\\' -> (
    match line.[i + 1] with
    | '\\'
    | '"' -> i + 2
    | 'x' -> i + 4
    | _ -> i + 1
  )
  | _ -> i + 1 in
  count_in_line i (curr_total + 1) line

let rec count_lines i curr_total_mem curr_total_char lines =
  let lines_len = List.length lines in
  if i >= lines_len then (curr_total_mem, curr_total_char) else
  let line = List.nth lines i in
  let line_total = count_in_line 0 0 line in
  count_lines (i + 1) (curr_total_mem + line_total) (curr_total_char + (String.length line)) lines

let rec encode_line i new_line_len og_line =
  let line_len = String.length og_line in
  if i >= line_len then new_line_len + 2 else
  let to_add = match og_line.[i] with
  | '"' 
  | '\\' -> 2
  | _ -> 1 in
  encode_line (i + 1) (new_line_len + to_add) og_line

let rec encode_lines i curr_total lines =
  let lines_len = List.length lines in
  if i >= lines_len then curr_total else
  let line = List.nth lines i in
  let line_total = encode_line 0 0 line in
  encode_lines (i + 1) (curr_total + line_total) lines

let p1 lines = 
  let (total_mem, total_char) = count_lines 0 0 0 lines in
  total_char - total_mem

let p2 lines =
  let (_, total_char) = count_lines 0 0 0 lines in
  let encoded_len = encode_lines 0 0 lines in
  encoded_len - total_char

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
