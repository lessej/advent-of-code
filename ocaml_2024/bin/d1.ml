open Ocaml_2024.Lib

let filename = "input/d1.txt"

let parse_line line =
  let parts = String.split_on_char ' ' line in
  let parts = List.map (fun x -> String.trim x) parts in
  (int_of_string (List.nth parts 0), int_of_string (List.nth parts 3))

let rec add_to_list (left, right) i lines =
  if (i >= List.length lines) then (left, right) else
  let line = List.nth lines i in
  let (leftPart, rightPart) = parse_line line in
  let left = left @ [leftPart] in
  let right = right @ [rightPart] in
  add_to_list (left, right) (i + 1) lines

let rec count_diff (left, right) i total_count =
  if (i >= List.length left) then total_count else
  let leftPart = List.nth left i in
  let rightPart = List.nth right i in
  let diff = Int.abs (rightPart - leftPart) in
  count_diff (left, right) (i + 1) (total_count + diff)

let p1 lines = 
  let (left, right) = add_to_list ([], []) 0 lines in
  let left = List.sort compare left in
  let right = List.sort compare right in
  count_diff (left, right) 0 0

let rec pop_hash hash right i =
  if (i >= List.length right) then hash else
  let curr_key = List.nth right i in
  let curr_val = match Hashtbl.find_opt hash curr_key with
    | None -> 0
    | Some count -> count in
  Hashtbl.replace hash curr_key (curr_val + 1);
  pop_hash hash right (i + 1)

let rec calc_occur hash left i total_count =
  if (i >= List.length left) then total_count else
  let left_key = List.nth left i in
  let val_for_left = match Hashtbl.find_opt hash left_key with
    | None -> 0
    | Some x -> x in
  let left_total = left_key * val_for_left in
  calc_occur hash left (i + 1) (total_count + left_total)

let p2 lines =
  let (left, right) = add_to_list ([], []) 0 lines in
  let hash = Hashtbl.create 1000 in
  let hash = pop_hash hash right 0 in
  calc_occur hash left 0 0

let () = 
  let lines = read_lines filename in 
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ();
