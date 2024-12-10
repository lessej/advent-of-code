open Ocaml_2024.Lib

let filename = "input/d9.txt"

let base_int_of_char c =
  int_of_char c - int_of_char '0'

let make_repeat_list n c =
  let rec make res c =
    if c = 0 then res else
    make (n::res) (c-1)
  in
  make [] c

let expand line =
  let rec loop i j expanded =
    if i >= String.length line then expanded else
    let count = base_int_of_char line.[i] in
    let insert = make_repeat_list j count in
    if i mod 2 = 0 then
      loop (i+1) (j+1) (insert::expanded)
    else
      let insert = make_repeat_list (-1*count) count in
      loop (i+1) j (insert::expanded)
  in
  List.rev (List.flatten (loop 0 0 []))

let shift_left expanded =
  let rec shift i j res =
    if i > j || i >= List.length expanded then res else
      match List.nth expanded i with
      | num when num < 0 -> (
        match List.nth expanded j with
        | num when num < 0 -> shift i (j-1) res
        | num -> shift (i+1) (j-1) (num::res)
      )
      | num -> shift (i+1) j (num::res)
  in
  let j = List.length expanded - 1 in
  List.rev (shift 0 j [])

let sum_expanded line =
  let rec sum_parts i parts res =
    match parts with
    | [] -> res
    | curr::rest ->
      sum_parts (i+1) rest (res+(curr*i))
  in
  sum_parts 0 line 0

let p1 lines =
  let line = List.hd lines in
  let expanded = expand line  in
  let shifted = shift_left expanded in
  let res = sum_expanded shifted in
  res

let build_ds str =
  let rec build i pos files empties =
    if i >= String.length str then (files,empties) else
    if i mod 2 = 0 then
      let len = base_int_of_char str.[i] in
      Hashtbl.replace files (i/2) (pos,len);
      build (i+1) (pos+len) files empties
    else
      let len = base_int_of_char str.[i] in
      let empties = (pos,len)::empties in
      build (i+1) (pos+len) files empties
  in
  build 0 0 (Hashtbl.create 100) []

let move_files files empties =
  let rec move i =
    if i < 0 then files else
    let (pos,size) = Hashtbl.find files i in
    let rec iter j =
      if j >= Array.length empties then () else
      let (start,len) = empties.(j) in
      if (start,len) = (-1,-1) then iter (j+1) else
      if start >= pos then () else
      if size > len then iter (j+1) else
      let () = Hashtbl.replace files i (start,size) in
      if size = len then 
        let () = empties.(j) <- (-1,-1) in
        iter Int.max_int
      else
        let () = empties.(j) <- (start+size,len-size) in
        iter Int.max_int
    in
    let () = iter 0 in
    move (i-1)
  in
  move (Hashtbl.length files - 1)

let sum_files files =
  let rec add_in_range s l m total =
    if s >= l then total else
    add_in_range (s+1) l m (total+s*m)
  in
  let rec sum i total =
    if i >= Hashtbl.length files then total else
    let (pos,len) = Hashtbl.find files i in
    let file_checksum = add_in_range pos (pos+len) i 0 in
    sum (i+1) (total+file_checksum)
  in
  sum 0 0

let p2 lines =
  let line = List.hd lines in
  let (files,empties) = build_ds line in
  let empties = List.rev empties |> Array.of_list in
  let moved_files = move_files files empties in
  sum_files moved_files

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
