open Ocaml_2024.Lib

let filename = "input/d11.txt"

let halves_of_string s =
  let len = String.length s in
  let fh = String.sub s 0 (len/2) in
  let sh = String.sub s (len/2) (len-(len/2)) in
  (fh,sh)

let parse_lines lines =
  let rec parse lines output =
    match lines with
    | [] -> output
    | curr::rest ->
      let output = String.split_on_char ' ' curr |> List.map (fun s -> int_of_string s) |> List.append output in
      parse rest output
  in
  parse lines []

let rec blink_stone bc hm stone =
  if bc = 0 then 1 else
  match Hashtbl.find_opt hm (bc,stone) with
  | Some ct -> ct
  | None ->
    let curr = match stone with
      | 0 -> blink_stone (bc-1) hm 1
      | s when (String.length (string_of_int s)) mod 2 = 0 ->
        let s = string_of_int s in
        let (fh,sh) = halves_of_string s in
        blink_stone (bc-1) hm (int_of_string fh) |> Int.add (blink_stone (bc-1) hm (int_of_string sh))
      | other -> blink_stone (bc-1) hm (other*2024)
    in
    Hashtbl.replace hm (bc,stone) curr;
    curr

let blink_stones times stones =
  let hm = Hashtbl.create 1000 in
  List.fold_left (fun acc stone ->
    blink_stone times hm stone |> Int.add acc
  ) 0 stones

let p1 lines =
  parse_lines lines |> blink_stones 25

let p2 lines =
  parse_lines lines |> blink_stones 75

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
