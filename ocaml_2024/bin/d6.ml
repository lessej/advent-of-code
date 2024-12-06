open Ocaml_2024.Lib

let filename = "input/d6.txt"

type dir =
  | Up
  | Down
  | Left
  | Right

let dir_of_char c =
  match c with
  | '<' -> Left
  | '>' -> Right
  | '^' -> Up
  | 'v' -> Down
  | unexpected -> 
    Printf.printf "unexpected direction char: %c\n " unexpected;
    failwith "unexpected direction character"

let get_next_dir dir =
  match dir with
  | Up -> Right
  | Down -> Left
  | Left -> Up
  | Right -> Down

let get_next_coord (x,y) dir =
  match dir with 
  | Up -> (x,y-1)
  | Down -> (x,y+1)
  | Left -> (x-1,y)
  | Right -> (x+1,y)

let to_matrix lines =
  let rec matrix_line rem arr_list =
    match rem with
    | [] -> arr_list
    | h::t ->
      let char_arary = Array.init (String.length h) (fun i -> h.[i]) in
      matrix_line t (char_arary :: arr_list)
  in
  Array.of_list (matrix_line (List.rev lines) [])
      
let get_visited (start_x,start_y) matrix =
  let out_of_bounds (x,y) =
    x < 0 || x >= Array.length matrix.(0) || y < 0 || y >= Array.length matrix
  in
  let rec follow (x,y) dir visited = 
    let (next_x,next_y) = get_next_coord (x,y) dir in
    if out_of_bounds (next_x,next_y) then 
      let () = Hashtbl.replace visited (x,y) true in
      visited
    else
      match matrix.(next_y).(next_x) with
      | '#' ->
        let dir = get_next_dir dir in
        follow (x,y) dir visited
      | _ ->
        Hashtbl.replace visited (x,y) true;
        follow (next_x,next_y) dir visited
  in
  let start_dir = dir_of_char matrix.(start_y).(start_x) in
  follow (start_x,start_y) start_dir (Hashtbl.create 100)

let add_loops (start_x,start_y) matrix visited =
  let out_of_bounds (x,y) =
    x < 0 || x >= Array.length matrix.(0) || y < 0 || y >= Array.length matrix
  in
  let rec check_for_loop (x,y) dir visited_this_loop =
    if Option.is_some (Hashtbl.find_opt visited_this_loop (x,y,dir)) then true else
    let (next_x,next_y) = get_next_coord (x,y) dir in
    if out_of_bounds (next_x,next_y) then 
      false
    else
      match matrix.(next_y).(next_x) with
      | '#' | 'O' ->
        let dir = get_next_dir dir in
        check_for_loop (x,y) dir visited_this_loop
      | _ ->
        Hashtbl.replace visited_this_loop (x,y,dir) true;
        check_for_loop (next_x,next_y) dir visited_this_loop
  in
  let rec try_change_curr rem make_loops =
    match rem with
    | [] -> make_loops
    | h::t ->
      let (try_x,try_y) = h in
      let prev = matrix.(try_y).(try_x) in
      Array.set matrix.(try_y) try_x 'O';
      if check_for_loop (start_x,start_y) Up (Hashtbl.create 100) then
        Hashtbl.replace make_loops (try_x,try_y) true;
      Array.set matrix.(try_y) try_x prev;
      try_change_curr t make_loops
  in
  let visited_vals = list_of_hash_keys visited in
  try_change_curr visited_vals (Hashtbl.create 100)

let find_start lines =
  let rec search rem (x,y) =
    match rem with
    | [] -> (x,y)
    | h::t ->
      match String.index_opt h '^' with
      | None -> search t (x,y+1)
      | Some x -> search [] (x,y)
  in
  search lines (0,0)

let p1 lines =
  let matrix = to_matrix lines in
  let (start_x,start_y) = find_start lines in
  let visited = get_visited (start_x, start_y) matrix in
  Hashtbl.length visited

let p2 lines =
  let matrix = to_matrix lines in
  let (start_x,start_y) = find_start lines in
  let visited = get_visited (start_x,start_y) matrix in
  let will_make_loops = add_loops (start_x,start_y) matrix visited in
  Hashtbl.length will_make_loops

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
