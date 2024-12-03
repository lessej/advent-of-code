open Ocaml_2024.Lib

let filename = "input/d3.txt"

let extract_muls contents =
  let rec take_mul i mul_str =
    match i with
    | i when i >= String.length contents -> None
    | i -> (
      match contents.[i] with
      | '0'..'9' | '(' | ',' | 'm' | 'u' | 'l' -> take_mul (i + 1) (mul_str ^ String.make 1 contents.[i])
      | ')' -> Some (mul_str ^ ")")
      | _ -> None
    )
  in
  let rec take_do i do_str =
    match i with
    | i when i >= String.length contents -> None
    | i -> (
      match contents.[i] with
      | 'd' | 'o' | '(' -> take_do (i + 1) (do_str ^ String.make 1 contents.[i])
      | ')' -> 
        let do_str = (do_str ^ ")") in
        if String.equal do_str "do()" then Some (do_str) else None
      | _ -> None
    )
  in
  let rec take_dont i dont_str =
    match i with
    | i when i >= String.length contents -> None
    | i -> (
      match contents.[i] with
      | 'd' | 'o' | 'n' | '\'' | 't' | '(' -> take_dont (i + 1) (dont_str ^ String.make 1 contents.[i])
      | ')' ->
        let dont_str = (dont_str ^ ")") in 
        if String.equal dont_str "don\'t()" then Some dont_str else None
      | _ -> None
    )
  in
  let rec find_all i is_on muls =
    match i with
    | i when i >= String.length contents -> muls
    | i -> (
      match contents.[i] with
      | 'd' -> 
        let is_on = if Option.is_some (take_do i "") then true else is_on in
        let is_on = if Option.is_some (take_dont i "") then false else is_on in
        find_all (i + 1) is_on muls
      | 'm' -> 
        let muls = match (take_mul i "") with
          | None -> muls
          | Some new_mul -> 
            if is_on then new_mul :: muls else muls
        in
        find_all (i + 1) is_on muls
      | _ -> find_all (i + 1) is_on muls
    )
  in
  find_all 0 true []

let total_muls muls =
  let rec add_to muls total =
    match muls with
    | [] -> total
    | h :: muls -> 
      if String.starts_with ~prefix:"mul(" h then
        let idx_open = String.index h '(' in
        let idx_split = String.index h ',' in
        let idx_close = String.index h ')' in
        if idx_open >= 0 && idx_split >= 0 && idx_close >= 0 then
          let num_1 = String.sub h (idx_open + 1) (idx_split - idx_open - 1) in
          let num_2 = String.sub h (idx_split + 1) (idx_close - idx_split - 1) in
          let total = total + (int_of_string num_1) * (int_of_string num_2) in
          add_to muls total
        else add_to muls total
      else add_to muls total
  in
  add_to muls 0

let p1 contents =
  let muls = extract_muls contents in
  total_muls muls

let () = 
  let contents = read_whole_file filename in 
  let p1_res = p1 contents in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  (* let p2_res = count_safe_with_removal lines in *)
  (* Printf.printf "The answer for part 2 is: %d\n" p2_res; *)
  ();
