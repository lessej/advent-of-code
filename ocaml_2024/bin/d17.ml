open Ocaml_2024.Lib

let filename = "input/d17.txt"

type op =
  | Adv of int
  | Bxl of int
  | Bst of int
  | Jnz of int
  | Bxc of int
  | Out of int
  | Bdv of int
  | Cdv of int

let string_of_op o =
  match o with
  | Adv v -> "Adv: " ^ (string_of_int v)
  | Bxl v -> "Bxl: " ^ (string_of_int v)
  | Bst v -> "Bst: " ^ (string_of_int v)
  | Jnz v -> "Jnz: " ^ (string_of_int v)
  | Bxc v -> "Bxc: " ^ (string_of_int v)
  | Out v -> "Out: " ^ (string_of_int v)
  | Bdv v -> "Bdv: " ^ (string_of_int v)
  | Cdv v -> "Cdv: " ^ (string_of_int v)

let op_of_strs o v =
  let v = int_of_string v in
  match o with
  | "0" -> Adv v
  | "1" -> Bxl v
  | "2" -> Bst v
  | "3" -> Jnz v
  | "4" -> Bxc v
  | "5" -> Out v
  | "6" -> Bdv v
  | "7" -> Cdv v
  | _ -> failwith "unexpected operation"

let get_registers lines =
  let parse_line line =
    String.length line - 12 |> String.sub line 12 |> int_of_string
  in
  let a = List.nth lines 0 |> parse_line in
  let b = List.nth lines 1 |> parse_line in
  let c = List.nth lines 2 |> parse_line in
  (a,b,c)

let get_instructions lines =
  let line = List.length lines - 1 |> List.nth lines in
  let parts = String.length line - 9 |> String.sub line 9 |> String.split_on_char ',' in
  let rec parse i res =
    if i >= List.length parts then res else
    (op_of_strs (List.nth parts i) (List.nth parts (i+1)))::res |> parse (i+2)
  in
  parse 0 [] |> List.rev

let compute_all instructions a b c =
  let rec compute i a b c res =
    if i >= List.length instructions then res else
    let curr = List.nth instructions i in
    let get_combo co =
      if co <= 3 then co else
      match co with
      | 4 -> a
      | 5 -> b
      | 6 -> c
      | _ -> failwith "unexpected combo operator"
    in
    match curr with
      | Adv v -> 
          let a = get_combo v |> pow 2 |> Int.div a in
          compute (i+1) a b c res
      | Bxl v ->
        let b = Int.logxor v b in
        compute (i+1) a b c res
      | Bst v -> 
          let v = get_combo v in
          let b = v mod 8 in
          compute (i+1) a b c res
      | Jnz v ->
          if a = 0 then compute (i+1) a b c res 
          else compute (v/2) a b c res
      | Bxc _v ->
          let b = Int.logxor b c in 
          compute (i+1) a b c res
      | Out v ->
          let v = get_combo v in
          let r = v mod 8 in
          r::res |> compute (i+1) a b c
      | Bdv v ->
          let b = get_combo v |> pow 2 |> Int.div a in
          compute (i+1) a b c res
      | Cdv v -> 
          let c = get_combo v |> pow 2 |> Int.div a in
          compute (i+1) a b c res
  in
  compute 0 a b c [] |> List.rev 

let p2 lines =
  let instructions = get_instructions lines in
  let quine = "2415751603465530" in
  let (a,b,c) = get_registers lines in
  let rec try_build_sol i curr res =
    if i >= 16 then Some res else
    let rec try_bits = function
      | -1 -> None
      | n ->
          let j = 7-n in
          let new_curr = curr+j in
          let otpt = compute_all instructions new_curr b c |> List.hd in
          let instr = String.sub quine (15-i) 1 |> int_of_string in
          if instr = otpt then
            match try_build_sol (i+1) (new_curr*8) (j::res) with
            | Some cont -> Some cont
            | None -> try_bits (n-1)
          else
            try_bits (n-1)
    in
    try_bits 7
  in
  let ans = match try_build_sol 0 0 [] with
    | None -> failwith "didn't find answer"
    | Some ans -> List.rev ans
        |> List.fold_left (fun acc x ->
          let acc = acc+x in
          acc*8
        ) 0
  in
  ans / 8

let p1 lines =
  let instructions = get_instructions lines in
  let (a,b,c) = get_registers lines in
  compute_all instructions a b c |> List.fold_left (fun acc r ->
    let str = string_of_int r in
    if acc = "" then str 
    else acc^","^str
  ) ""

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %s\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
