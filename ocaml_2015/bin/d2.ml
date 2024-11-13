let calc_line line = 
  let parts = String.split_on_char 'x' line in
  let vals = List.map (fun x -> int_of_string x) parts in
  let ordered = List.sort compare vals in
  let first = List.nth ordered 0 in
  let second = List.nth ordered 1 in
  let third = List.nth ordered 2 in
  let base_sa = 2 * (first * second + first * third + second * third) in
  let extra_sa = first * second in
  base_sa + extra_sa

let rec calc_total accum idx all_lines = 
  if idx >= List.length all_lines then accum
  else
    let line = List.nth all_lines idx in
    let line_total = calc_line line in
    calc_total (accum + line_total) (idx + 1) all_lines

let calc_ribbon line =
  let parts = List.sort compare (
    List.map (fun x -> int_of_string x) (
      String.split_on_char 'x' line
    )
  ) in
  2 * (List.nth parts 0 + List.nth parts 1) + (List.nth parts 0 * List.nth parts 1 * List.nth parts 2)

let rec calc_total_ribbon accum idx all_lines = 
  if idx >= List.length all_lines then accum
  else
    let line_total = calc_ribbon (List.nth all_lines idx) in
    calc_total_ribbon (accum + line_total) (idx + 1) all_lines

let rec read_lines ic =
  try
    let line = input_line ic in
    line :: read_lines ic
  with
    End_of_file -> []

let () = 
  let ic = open_in "bin/input/d2.txt" in
  let all_lines = read_lines ic in
  let total = calc_total 0 0 all_lines in
  Printf.printf "The total is %d\n" total;
  let total_p2 = calc_total_ribbon 0 0 all_lines in
  Printf.printf "The total for part 2 is %d\n" total_p2;
