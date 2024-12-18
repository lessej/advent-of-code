open Ocaml_2024.Lib

let filename = "input/d15.txt"

let parse_grid lines =
  let rec get lines grid_lines =
    match lines with
    | [] -> grid_lines
    | curr::rest ->
        if String.equal curr "" then
          get [] grid_lines
        else
          curr::grid_lines |> get rest
  in
  get lines [] |> List.rev |> matrix_of_string_list

let parse_instructions lines =
  let middle = match List.find_index (fun l -> String.equal l "") lines with
  | Some m -> m
  | None -> failwith "couldn't find middle"
  in
  let rec get i instructions =
    if i >= List.length lines then instructions else
    let line = List.nth lines i in
    get (i+1) (instructions^line)
  in
  get (middle+1) "" |> String.to_seq |> List.of_seq

let get_start grid =
  let rec search x y =
    if x >= Array.length grid.(0) then
      search 0 (y+1)
    else
      if y >= Array.length grid then (-1,-1) else
      if grid.(y).(x) = '@' then (x,y) else
      search (x+1) y
  in
  search 0 0

let sum_boxes grid =
  let rec search x y sum =
    if x >= Array.length grid.(0) then
      search 0 (y+1) sum
    else
      if y >= Array.length grid then sum else
      if grid.(y).(x) = 'O' then 
        100*y+x |> Int.add sum |> search (x+1) y 
      else
        search (x+1) y sum
  in
  search 0 0 0

let move_horiz px py matrix op =
  let rec find_start i =
    match matrix.(py).(i) with
    | '#' -> None
    | '.' -> Some i
    | _ -> find_start (op i 1)
  in
  match find_start (op px 1) with
  | None -> (px,py)
  | Some start ->
      matrix.(py).(start) <- 'O';
      matrix.(py).(op px 1) <- '@';
      matrix.(py).(px) <- '.';
      (op px 1,py)

let move_vert px py matrix op =
  let rec find_start i =
    match matrix.(i).(px) with
    | '#' -> None
    | '.' -> Some i
    | _ -> find_start (op i 1)
  in
  match find_start (op py 1) with
  | None -> (px,py)
  | Some start ->
      matrix.(start).(px) <- 'O';
      matrix.(op py 1).(px) <- '@';
      matrix.(py).(px) <- '.';
      (px,op py 1)

let move_all matrix sx sy instructions =
  let rec do_instr rem cx cy =
    match rem with
    | [] -> matrix
    | curr::rest -> (
        let (cx,cy) = match curr with
          | '<' -> move_horiz cx cy matrix (-) 
          | '>' -> move_horiz cx cy matrix (+) 
          | '^' -> move_vert cx cy matrix (-) 
          | 'v' -> move_vert cx cy matrix (+) 
          | _ -> failwith "unexpected direction"
        in
        do_instr rest cx cy
    )
  in
  do_instr instructions sx sy

let move_double_horiz px py matrix op =
  let rec find_start i =
    match matrix.(py).(i) with
    | '#' -> None
    | '.' -> Some i
    | _ -> find_start (op i 1)
  in
  let rec flip_l s e =
    matrix.(py).(s) <- matrix.(py).(s+1);
    let s = s+1 in
    if s = e then () else
    flip_l s e
  in
  let rec flip_r s e =
    matrix.(py).(s) <- matrix.(py).(s-1);
    let s = s-1 in
    if s = e then () else
    flip_r s e
  in
  match find_start (op px 1) with
  | None -> (px,py)
  | Some start ->
      let () = if start < (px-1) then
        flip_l start (op px 1)
      else if start > (px+1) then
        flip_r start (op px 1)
      else
        ()
      in
      matrix.(py).(op px 1) <- '@';
      matrix.(py).(px) <- '.';
      (op px 1,py)

let move_double_vert px py matrix op =
  let rec can_move_up i l r =
    if matrix.(i).(l) = '.' && matrix.(i).(r) = '.' then true else
    if matrix.(i).(l) = '#' || matrix.(i).(r) = '#' then false else
    if matrix.(i).(l) = '[' then can_move_up (op i 1) l r else
    let ok_l = if matrix.(i).(l) = ']' then can_move_up (op i 1) (l-1) l else true 
    in
    let ok_r = if matrix.(i).(r) = '[' then can_move_up (op i 1) r (r+1) else true 
    in
    ok_l && ok_r
  in
  let rec get_movable i l r =
    if matrix.(i).(l) = '.' && matrix.(i).(r) = '.' then [] else
    if matrix.(i).(l) = '[' then 
      (i,l,'[')::(i,r,']')::(get_movable (op i 1) l r) 
    else
      let ok_l = if matrix.(i).(l) = ']' then 
        (i,l-1,'[')::(i,l,']')::(get_movable (op i 1) (l-1) l) else [] 
      in
      let ok_r = if matrix.(i).(r) = '[' then 
        (i,r,'[')::(i,r+1,']')::(get_movable (op i 1) r (r+1)) else [] 
      in
      (ok_l@ok_r)
  in
  if matrix.(op py 1).(px) = '.' then
    let () = matrix.(py).(px) <- '.' in
    let () = matrix.(op py 1).(px) <- '@' in
    (px,op py 1)
  else
  let ok_move = if matrix.(op py 1).(px) = '[' then
    can_move_up (op py 1) px (px+1)
  else
    can_move_up (op py 1) (px-1) px
  in
  if ok_move then
    let movable = if matrix.(op py 1).(px) = '[' then
      get_movable (op py 1) px (px+1)
    else
      get_movable (op py 1) (px-1) px
    in
    let hm = Hashtbl.create 10 in
    List.rev movable |> List.iter (fun (y,x,c) ->
      match Hashtbl.find_opt hm (x,(op y 1)) with
      | None -> 
          Hashtbl.replace hm (x,(op y 1)) true;
          matrix.(y).(x) <- '.';
          matrix.(op y 1).(x) <- c;
      | Some _v -> ()
    );
    let () = matrix.(py).(px) <- '.' in
    let () = matrix.(op py 1).(px) <- '@' in
    (px,op py 1)
  else
    (px,py)

let expand_grid grid =
  let rec iter x y curr_line expanded =
    if x >= Array.length grid.(0) then
      curr_line::expanded |> iter 0 (y+1) ""
    else
      if y >= Array.length grid then expanded else
      let to_add = match grid.(y).(x) with
        | '.' -> ".."
        | '#' -> "##"
        | '@' -> "@."
        | 'O' -> "[]"
        | _ -> failwith "unexpected char"
      in
      iter (x+1) y (curr_line^to_add) expanded
  in
  iter 0 0 "" [] |> List.rev |> matrix_of_string_list

let move_all_double matrix sx sy instructions =
  let rec do_instr rem cx cy =
    match rem with
    | [] -> matrix
    | curr::rest -> (
        let (cx,cy) = match curr with
          | '<' -> move_double_horiz cx cy matrix (-) 
          | '>' -> move_double_horiz cx cy matrix (+) 
          | '^' -> move_double_vert cx cy matrix (-) 
          | 'v' -> move_double_vert cx cy matrix (+) 
          | _ -> failwith "unexpected direction"
        in
        do_instr rest cx cy
    )
  in
  do_instr instructions sx sy

let sum_double_boxes grid =
  let rec search x y sum =
    if x >= Array.length grid.(0) then
      search 0 (y+1) sum
    else
      if y >= Array.length grid then sum else
      if grid.(y).(x) = '[' then 
        100*y+x |> Int.add sum |> search (x+1) y 
      else
        search (x+1) y sum
  in
  search 0 0 0

let p1 lines =
  let matrix = parse_grid lines in
  let instructions = parse_instructions lines in
  let (sx,sy) = get_start matrix in
  let matrix = move_all matrix sx sy instructions in
  sum_boxes matrix

let p2 lines =
  let matrix = parse_grid lines in
  let instructions = parse_instructions lines in
  let expanded_grid = expand_grid matrix in
  let (sx,sy) = get_start expanded_grid in
  let expanded_grid = move_all_double expanded_grid sx sy instructions in
  sum_double_boxes expanded_grid

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
