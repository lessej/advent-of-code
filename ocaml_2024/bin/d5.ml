open Ocaml_2024.Lib

let filename = "input/d5.txt"

let to_input_parts lines =
  let rec split_input rem is_tree tree pages =
    match rem with
    | [] -> (tree, pages)
    | h::rem -> (
      if (String.equal h "") then 
        split_input rem false tree pages 
      else
        match is_tree with
        | true -> split_input rem is_tree (h::tree) pages
        | false -> split_input rem is_tree tree (h::pages)
    )
  in
  split_input lines true [] []

let build_tree tree =
  let parse_line line =
    let parts = String.split_on_char '|' line in
    let less = List.nth parts 0 in
    let more = List.nth parts 1 in
    (int_of_string less, int_of_string more)
  in
  let rec add_line rem ht =
    match rem with
    | [] -> ht
    | h::rem -> (
      let (less, more) = parse_line h in
      let curr = match Hashtbl.find_opt ht more with
        | None -> [less]
        | Some l -> less::l
      in
      Hashtbl.replace ht more curr;
      add_line rem ht
    )
  in
  let ht = Hashtbl.create 50 in
  add_line tree ht

let list_of_page page =
  let parts = String.split_on_char ',' page in
  List.map (fun x -> int_of_string x) parts

let sum_middle ok_lines =
  let rec sum ok_lines total =
    match ok_lines with
    | [] -> total
    | h::rest ->
      let line_len = List.length h in
      let half_idx = line_len / 2 in
      let half = List.nth h half_idx in
      sum rest (total + half)
  in
  sum ok_lines 0

let get_not_ok_lines lines tree = 
  let rec check_line rem not_ok =
    match rem with
    | [] -> not_ok
    | h::rem -> (
      match Hashtbl.find_opt tree h with
      | None -> check_line rem not_ok
      | Some lesser -> (
        let not_ok = List.append (List.filter (fun x ->
          Option.is_some (List.find_opt (fun y -> y == x) lesser)
        ) rem) not_ok
        in
        check_line rem not_ok
      )
    )
  in
  let rec check_lines rem not_ok_lines =
    match rem with
    | [] -> not_ok_lines
    | h::rem -> (
      let line = list_of_page h in
      let not_ok = check_line line [] in
      let not_ok_lines = if List.length not_ok <= 0 then 
        not_ok_lines
      else 
        line::not_ok_lines
      in
      check_lines rem not_ok_lines
    )
  in
  check_lines lines []

let fix_not_ok_lines lines tree =
  let rec insert x l =
    match l with
    | [] -> [x]
    | y::ys ->
      match Hashtbl.find_opt tree y with
      | None -> x::y::ys
      | Some lesser -> (
        if Option.is_some (List.find_opt (fun m -> m == x) lesser) then
          x::y::ys
        else
          y::insert x ys
      )
  in
  let rec sort l =
    match l with
    | [] -> []
    | x::xs -> insert x (sort xs)
  in
  List.map (fun l -> sort l) lines
      
let p1 pages tree = 
  let not_ok_lines = get_not_ok_lines pages tree in
  let total_sum = sum_middle (List.map (fun x -> list_of_page x) pages) in
  let not_ok_sum = sum_middle not_ok_lines in
  total_sum - not_ok_sum

let p2 pages tree =
  let not_ok_lines = get_not_ok_lines pages tree in
  let fixed = fix_not_ok_lines not_ok_lines tree in
  sum_middle fixed

let () = 
  let lines = read_lines filename in 
  let (tree, pages) = to_input_parts lines in
  let tree = build_tree tree in
  let p1_res = p1 pages tree in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 pages tree in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
