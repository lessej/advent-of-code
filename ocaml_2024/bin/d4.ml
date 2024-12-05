open Ocaml_2024.Lib

let filename = "input/d4.txt"

type dir =
  | Tl
  | Tc
  | Tr
  | Ml
  | Mr
  | Bl
  | Bc
  | Br

let string_of_dir dir =
  match dir with
  | Tl -> "Tl"
  | Tc -> "Tc"
  | Tr -> "Tr"
  | Ml -> "Ml"
  | Mr -> "Mr"
  | Bl -> "Br"
  | Bc -> "Bc"
  | Br -> "Bl"

let get_next (x,y) d = 
  match d with
  | Tl -> (x-1,y-1)
  | Tc -> (x,y-1)
  | Tr -> (x+1,y-1)
  | Ml -> (x-1,y)
  | Mr -> (x+1,y)
  | Bl -> (x-1,y+1)
  | Bc -> (x,y+1)
  | Br -> (x+1,y+1)

let find_xmas lines =
  let line_len = String.length (List.nth lines 0) in
  let col_len = List.length lines in
  let rec follow (x,y) d curr =
    if String.equal curr "XMAS" then 1 else
    if x < 0 || x >= line_len || y < 0 || y >= col_len then 0 else
    let line = List.nth lines y in
    match line.[x] with
    | c when (Char.equal c 'M' || Char.equal c 'A' || Char.equal c 'S' || Char.equal c 'X') ->
      let curr = curr ^ (String.make 1 c) in
      let (x,y) = get_next (x,y) d in
      follow (x,y) d curr
    | _ -> 0
  in
  let rec follow_all (x,y) dirs count =
    match dirs with
    | [] -> count
    | curr::rest -> (
      let count = count + follow (x,y) curr "" in
      follow_all (x,y) rest count
    )
  in
  let rec scan_grid lines y count =
    match lines with
    | [] -> count
    | curr::rest -> (
      let rec scan x count =
        if x >= String.length curr then count else
        let count = match curr.[x] with
          | 'X' -> follow_all (x,y) [Tl; Tc; Tr; Ml; Mr; Bl; Bc; Br] count
          | _ -> count
        in
        scan (x + 1) count
      in
      let count = scan 0 count in
      scan_grid rest (y+1) count
    )
  in
  scan_grid lines 0 0

let find_masmas lines = 
  let check_diags (x,y) =
    let line_top = List.nth lines (y-1) in
    let line_bottom = List.nth lines (y+1) in
    let tl = line_top.[x-1] in
    let tr = line_top.[x+1] in
    let bl = line_bottom.[x-1] in
    let br = line_bottom.[x+1] in
    ((Char.equal tl 'M' && Char.equal br 'S') || (Char.equal tl 'S' && Char.equal br 'M')) &&
    ((Char.equal tr 'M' && Char.equal bl 'S') || (Char.equal tr 'S' && Char.equal bl 'M'))
  in
  let rec scan_grid y count =
    if y >= (List.length lines - 1) then count else
    let curr = List.nth lines y in
    let rec scan x count =
      if x >= (String.length curr - 1) then count else
      let count = match curr.[x] with
        | 'A' -> if check_diags (x,y) then count + 1 else count
        | _ -> count
      in
      scan (x + 1) count
    in
    let count = scan 1 count in
    scan_grid (y+1) count
  in
  scan_grid 1 0

let () = 
  let lines = read_lines filename in
  let p1_res = find_xmas lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = find_masmas lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ();
