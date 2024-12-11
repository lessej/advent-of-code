open Ocaml_2024.Lib

let filename = "input/d10.txt"

let base_int_of_char c =
  int_of_char c - int_of_char '0'

let get_raw_next_coords x y =
  (x-1,y)::(x,y-1)::(x+1,y)::(x,y+1)::[]

let find_trailends lines =
  let rec search_y y lines found =
    match lines with
    | [] -> found
    | curr::rest -> (
      let rec search_x x found_in_line =
        if x >= String.length curr then found_in_line else
        let in_line = match curr.[x] with
          | '9' -> (x,y)::found_in_line
          | _ -> found_in_line
        in
        search_x (x+1) in_line
      in
      search_x 0 [] |> List.append found |> search_y (y+1) rest
    )
  in
  search_y 0 lines []

let find_trailheads x y matrix =
  let out_of_bounds x y =
    x >= Array.length matrix.(0) || x < 0 || y >= Array.length matrix || y < 0
  in
  let get_next_coords x y =
    let curr_val = base_int_of_char matrix.(y).(x) in
    let check_coord nx ny =
      if out_of_bounds nx ny then None else
      if matrix.(ny).(nx) = '.' then None else
      let next_val = base_int_of_char matrix.(ny).(nx) in
      if curr_val - next_val != 1 then None else 
      Some (nx,ny)
    in
    get_raw_next_coords x y |> List.fold_left (fun acc (nx,ny) ->
      match check_coord nx ny with
      | None -> acc
      | Some coord -> coord::acc
    ) [] 
  in
  let rec search x y trailheads =
    if matrix.(y).(x) = '0' then
      (x,y)::trailheads
    else
      get_next_coords x y |> List.fold_left (fun acc (nx,ny) ->
        search nx ny trailheads |> List.append acc
      ) []
  in
  search x y []

let find_trails x y matrix =
  let out_of_bounds x y =
    x >= Array.length matrix.(0) || x < 0 || y >= Array.length matrix || y < 0
  in
  let get_next_coords x y =
    let curr_val = base_int_of_char matrix.(y).(x) in
    let check_coord nx ny =
      if out_of_bounds nx ny then None else
      if matrix.(ny).(nx) = '.' then None else
      let next_val = base_int_of_char matrix.(ny).(nx) in
      if curr_val - next_val != 1 then None else 
      Some (nx,ny)
    in
    get_raw_next_coords x y |> List.fold_left (fun acc (nx,ny) ->
      match check_coord nx ny with
      | None -> acc
      | Some coord -> coord::acc
    ) [] 
  in
  let rec search x y current_path trails =
    if matrix.(y).(x) = '0' then
      let x = string_of_int x in
      let y = string_of_int y in
      let current_path = current_path^"("^x^","^y^")" in
      current_path::trails
    else
      get_next_coords x y |> List.fold_left (fun acc (nx,ny) ->
        let snx = string_of_int nx in
        let sny = string_of_int ny in
        let current_path = current_path^"("^snx^","^sny^")" in
        search nx ny current_path trails |> List.append acc
      ) []
  in
  let sx = string_of_int x in
  let sy = string_of_int y in
  search x y ("("^sx^","^sy^")") []

let to_trail_head_hash trail_ends matrix =
  let rec add_trail_head ends hm =
    match ends with
    | [] -> hm
    | (tex,tey)::rest -> (
      let trail_heads = find_trailheads tex tey matrix in
      List.iter (fun (thx,thy) ->
        match Hashtbl.find_opt hm (thx,thy) with
        | None -> (
          let hhm = Hashtbl.create 10 in
          Hashtbl.replace hhm (tex,tey) true;
          Hashtbl.replace hm (thx,thy) hhm;
        )
        | Some hhm -> Hashtbl.replace hhm (tex,tey) true;
      ) trail_heads;
      add_trail_head rest hm
    )
  in
  Hashtbl.create 10 |> add_trail_head trail_ends

let p1 lines =
  let trail_ends = find_trailends lines in
  let matrix = matrix_of_string_list lines in
  let trail_heads = to_trail_head_hash trail_ends matrix in
  list_of_hash_vals trail_heads |> List.fold_left (fun acc hhm ->
    acc + (Hashtbl.length hhm)
  ) 0

let p2 lines =
  let trail_ends = find_trailends lines in
  let matrix = matrix_of_string_list lines in
  List.fold_left (fun acc (x,y) ->
    let trail_count = find_trails x y matrix |> List.length in
    acc+trail_count
  ) 0 trail_ends

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
