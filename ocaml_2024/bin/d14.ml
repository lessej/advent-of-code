open Ocaml_2024.Lib

let filename = "input/d14.txt"

let parse_lines lines =
  let rec parse lines res =
    match lines with
    | [] -> res
    | curr::rest ->
        let parts = String.split_on_char ' ' curr 
          |> List.map (fun s -> String.trim s)
          |> List.map (fun s ->
              let nums = String.length s - 2
                |> String.sub s 2
                |> String.split_on_char ','
                |> List.map (fun s -> int_of_string s)
              in
              (List.nth nums 0, List.nth nums 1)
          )
        in
        (List.nth parts 0, List.nth parts 1)::res |> parse rest
  in
  parse lines []

let wrap a b =
  let a = a mod b |> Int.add b in
  a mod b

let final_pos sx sy vx vy w h frames =
  let final_x = wrap (sx+vx*frames) w in
  let final_y = wrap (sy+vy*frames) h in
  (final_x,final_y)

let add_to_quads w h finals =
  let midw = w/2 in
  let midh = h/2 in
  let get_quad x y =
    match (x,y) with
    | (x,y) when x < midw && y < midh -> Some(1)
    | (x,y) when x > midw && y < midh -> Some(2)
    | (x,y) when x < midw && y > midh -> Some(3)
    | (x,y) when x > midw && y > midh -> Some(4)
    | _ -> None
  in
  let rec into finals q1 q2 q3 q4 =
    match finals with
    | [] -> q1::q2::q3::q4::[]
    | (x,y)::rest -> (
        match get_quad x y with
        | None -> into rest q1 q2 q3 q4
        | Some(1) -> into rest (q1+1) q2 q3 q4
        | Some(2) -> into rest q1 (q2+1) q3 q4
        | Some(3) -> into rest q1 q2 (q3+1) q4
        | Some(4) -> into rest q1 q2 q3 (q4+1)
        | _-> failwith "Unexpected option"
    )
  in
  into finals 0 0 0 0

let get_new_pos s v =
  List.map2 (fun (sx,sy) (vx,vy) ->
    final_pos sx sy vx vy 101 103 1
  ) s v

let print_grid s =
  let hm = Hashtbl.create 500 in
  List.iter (fun p ->
    match Hashtbl.find_opt hm p with
    | None -> Hashtbl.replace hm p 1
    | Some curr -> Hashtbl.replace hm p (curr+1);
  ) s;
  for y = 0 to 103 do
    for x = 0 to 101 do
      let c = match Hashtbl.find_opt hm (x,y) with
      | None -> '.'
      | Some _v -> '#'
      in
      Printf.printf "%c" c;
    done;
    Printf.printf "\n";
  done;
  Printf.printf "\n\n";
  ()

let check_low_safety_score s v =
  let rec check i s lowest l =
    if i >= (101*103) then (l+1) else
    let s = get_new_pos s v in
    let ss = add_to_quads 101 103 s
      |> List.fold_left (fun acc q ->
          acc*q
      ) 1
    in
    if ss <= lowest then
      let () = Printf.printf "I: %d\n\n" i in
      let () = print_grid s in
      check (i+1) s ss i
    else
      check (i+1) s lowest l
  in
  check 0 s Int.max_int 0

let p2 lines =
  let (s,v) = parse_lines lines |> List.split in
  check_low_safety_score s v

let p1 lines =
  parse_lines lines 
    |> List.map (fun ((sx,sy),(vx,vy)) ->
        final_pos sx sy vx vy 101 103 100
    ) 
    |> add_to_quads 101 103
    |> List.fold_left (fun acc q ->
        acc*q
    ) 1

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()



(* let move_double_up px py matrix = *)
(*   let rec can_move_up i l r = *)
(*     if matrix.(i).(l) = '.' && matrix.(i).(r) = '.' then true else *)
(*     if matrix.(i).(l) = '#' || matrix.(i).(r) = '#' then false else *)
(*     if matrix.(i).(l) = '[' then can_move_up (i-1) l r else *)
(*     let ok_l = if matrix.(i).(l) = ']' then can_move_up (i-1) (l-1) l else true in *)
(*     let ok_r = if matrix.(i).(r) = '[' then can_move_up (i-1) r (r+1) else true in *)
(*     ok_l && ok_r *)
(*   in *)
(*   can_move_up 4 6 7  *)
(*  *)
