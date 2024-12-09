open Ocaml_2024.Lib

let filename = "input/d8.txt"

let find_antennas lines =
  let rec search mat_lines y ants =
    match mat_lines with
    | [] -> list_of_hash_vals ants
    | mat_line::rest -> (
      let () = String.iteri (fun x c -> 
        match c with
        | c when (not (Char.equal '.' c) && not (Char.equal '#' c)) ->
          let found = match Hashtbl.find_opt ants c with
            | None -> []
            | Some found -> found
          in
          let found = (x,y)::found in
          Hashtbl.replace ants c found
        | _ -> ()
      ) mat_line
      in
      search rest (y+1) ants
    )
  in
  search lines 0 (Hashtbl.create 20)

let rem_dupes antis =
  let rec rem_using_hm rest hm =
    match rest with 
      | [] -> list_of_hash_keys hm
      | curr::rest ->
          Hashtbl.replace hm curr true;
        rem_using_hm rest hm
  in
  rem_using_hm antis (Hashtbl.create 50)

let find_antinodes antennas matrix =
  let check_out_of_bounds (x,y) =
    let x_len = Array.length matrix.(0) in
    let y_len = Array.length matrix in
    x < 0 || x >= x_len || y < 0 || y >= y_len
  in
  let rec get_all_antis (x,y) (i,j) =
    let (anti_x,anti_y) = (2*x-i,2*y-j) in
    match check_out_of_bounds (anti_x,anti_y) with
    | true -> []
    | false -> 
      [(anti_x,anti_y)]@(get_all_antis (anti_x,anti_y) (x,y))@[(x,y)]
  in
  let get_first_antis (x,y) (i,j) =
    (2*x-i,2*y-j)
  in
  let rec check_pairs_for_first ants antis =
    match ants with
    | [] -> antis
    | curr::rest ->
      let found = List.fold_left (fun acc_antis ant ->
        let try_0 = get_first_antis curr ant in
        let try_1 = get_first_antis ant curr in
        let try_0 = if check_out_of_bounds try_0 then [] else [try_0] in
        let try_1 = if check_out_of_bounds try_1 then [] else [try_1] in
        acc_antis@try_0@try_1
      ) [] rest
      in
      check_pairs_for_first rest antis@found
  in
  let rec check_pairs_for_all ants antis =
    match ants with
    | [] -> antis
    | curr::rest ->
      let found = List.fold_left (fun acc_antis ant ->
        let try_0 = get_all_antis curr ant in
        let try_1 = get_all_antis ant curr in
        acc_antis@try_0@try_1
      ) [] rest
      in
      check_pairs_for_all rest antis@found
  in
  let with_dupes = List.fold_left (fun acc_antis ants ->
    let found = check_pairs_for_first ants [] in
    acc_antis@found
  ) [] antennas
  in
  let with_dupes_all = List.fold_left (fun acc_antis ants ->
    let found = check_pairs_for_all ants [] in
    acc_antis@found
  ) [] antennas
  in
  (rem_dupes with_dupes, rem_dupes with_dupes_all)

let solve lines =
  let antennas = find_antennas lines in
  let matrix = matrix_of_string_list lines in
  let (first,all) = find_antinodes antennas matrix in
  let all_antennas = List.fold_left (fun acc l -> acc@l) [] antennas in
  let no_dupes = rem_dupes (all@all_antennas) in
  (List.length first,List.length no_dupes)

let () = 
  let lines = read_lines filename in
  let (p1_res,p2_res) = solve lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
