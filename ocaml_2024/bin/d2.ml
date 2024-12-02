open Ocaml_2024.Lib

let filename = "input/d2.txt"

type dir = 
  | Dec 
  | Inc
  | Same

let parse_line line =
  let parts = String.split_on_char ' ' line in
  let parts = List.map (fun x -> String.trim x) parts in
  List.map (fun x -> int_of_string x) parts

let det_line line = 
  let rec check_diff i d =
    if i >= (List.length line) then true else
    let j = i - 1 in
    let next = List.nth line i in
    let prev = List.nth line j in 
    let dir = match (next - prev) with
      | x when x < 0 -> Dec
      | x when x > 0 -> Inc
      | _ -> Same in
    if dir != d && dir != Same then false else
    let diff = Int.abs (next - prev) in
    if (diff < 1) || (diff > 3) then false else
    check_diff (i + 1) dir in
  let second = List.nth line 1 in
  let first = List.nth line 0 in 
  let first_dir = match (second - first) with
  | x when x < 0 -> Dec
  | x when x > 0 -> Inc 
  | _ -> Same in
  check_diff 1 first_dir

let count_safe lines =
  let rec count i total =
    if i >= List.length lines then total else
    let line = List.nth lines i in
    let parsed = parse_line line in
    let is_safe = det_line parsed in
    let to_add = match is_safe with
    | true -> 1
    | false -> 0 in
    count (i + 1) (total + to_add) in
  count 0 0

let det_with_removal line =
  let rec check_with_removal i =
    if i >= List.length line then false else
    let list_with_removal = List.filteri (fun idx _x -> idx != i) line in
    if det_line list_with_removal then true else check_with_removal (i + 1) in
  check_with_removal 0

let count_safe_with_removal lines = 
  let rec count i total =
    if i >= List.length lines then total else
    let line = List.nth lines i in
    let line = parse_line line in
    let is_safe = det_line line in
    let to_add = match is_safe with
      | true -> 1
      | false -> (
        match det_with_removal line with
        | true -> 1
        | false -> 0
      ) in
    count (i + 1) (total + to_add) in
  count 0 0

let () = 
  let lines = read_lines filename in 
  let p1_res = count_safe lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = count_safe_with_removal lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ();
