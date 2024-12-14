open Ocaml_2024.Lib

let filename = "input/d13.txt"

let to_float (x,y) =
  (float_of_int x, float_of_int y)

let parse_input lines =
  let parse_line pref_len line =
    let parts = String.length line - pref_len
      |> String.sub line pref_len 
      |> String.split_on_char ',' 
      |> List.map (fun s -> String.trim s)
      |> List.map (fun s ->
          String.length s - 2
            |> String.sub s 2
            |> int_of_string
         )
    in
    (List.nth parts 0, List.nth parts 1)
  in
  let rec parse i machines =
    if i >= List.length lines then machines else
    let ba = List.nth lines i |> parse_line 10 in
    let bb = List.nth lines (i+1) |> parse_line 10 in
    let p = List.nth lines (i+2) |> parse_line 7 in
    (p,ba,bb)::machines |> parse (i+4)
  in
  parse 0 []

let play_machine prize a b limit offset =
  let (px,py) = to_float prize in
  let px = px+.offset in
  let py = py+.offset in
  let (ax,ay) = to_float a in
  let (bx,by) = to_float b in
  let det = ax*.by-.bx*.ay in
  let deta = px*.by-.bx*.py in
  let detb = ax*.py-.px*.ay in
  let res_a = deta/.det in
  let res_b = detb/.det in
  if Float.is_integer res_a && Float.is_integer res_b then
    let res_a = int_of_float res_a in
    let res_b = int_of_float res_b in
    if (res_a > limit || res_b > limit) then None else
    Some (res_a,res_b)
  else
    None

let p1 lines =
  parse_input lines 
    |> List.map (fun (p,a,b) ->
        play_machine p a b 100 0.0
    )
    |> List.filter (fun r -> Option.is_some r) 
    |> List.map (fun r -> Option.get r)
    |> List.fold_left (fun acc (ra,rb) ->
        (ra*3+rb)+acc
    ) 0

let p2 lines =
  parse_input lines 
    |> List.map (fun (p,a,b) ->
        play_machine p a b Int.max_int 10000000000000.0
    )
    |> List.filter (fun r -> Option.is_some r) 
    |> List.map (fun r -> Option.get r)
    |> List.fold_left (fun acc (ra,rb) ->
        (ra*3+rb)+acc
    ) 0

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
