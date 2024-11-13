open Ocaml_2015.Lib

let filename = "bin/input/d6.txt"

type action_type = 
  | On
  | Off
  | Toggle

let action_from_string str =
  match str with 
    | _ when String.starts_with ~prefix:"turn on" str -> On
    | _ when String.starts_with ~prefix:"turn off" str -> Off
    | _ when String.starts_with ~prefix:"toggle" str -> Toggle
    | _ -> failwith "Unexpected action"

let perform action (start_x,start_y) (end_x,end_y) lights =
  for i = start_x to end_x do
    for j = start_y to end_y do
      match action with
        | On -> lights.(i).(j) <- true
        | Off -> lights.(i).(j) <- false
        | Toggle -> lights.(i).(j) <- not lights.(i).(j)
    done
  done;;

let change_brightness action (start_x,start_y) (end_x,end_y) lights =
  for i = start_x to end_x do
    for j = start_y to end_y do
      match action with
        | On -> lights.(i).(j) <- lights.(i).(j) + 1
        | Off -> 
          (let new_brightness = match (lights.(i).(j) - 1) with
          | x when x < 0  -> 0
          | x -> x in
          lights.(i).(j) <- new_brightness
          )
        | Toggle -> lights.(i).(j) <- lights.(i).(j) + 2
    done
  done;;

(* 
   extract action
   extract first coord
   extract second coor
   perform action
*)

let rec read_backward str i curr_str =
  if str.[i] = ' ' then curr_str else
  let add_char = String.make 1 str.[i] in
  let curr_str = add_char ^ curr_str in
  read_backward str (i-1) curr_str

let rec read_forward str i curr_str =
  if (i > String.length str - 1 || str.[i] = ' ') then curr_str else 
  let add_char = String.make 1 str.[i] in
  let curr_str = curr_str ^ add_char in
  read_forward str (i+1) curr_str

let get_coord str comma_idx =
  let x_coord = read_backward str (comma_idx-1) "" in
  let x_coord = match int_of_string_opt x_coord with
  | None -> failwith "Invalid x coordinate"
  | Some num -> num in
  let y_coord = read_forward str (comma_idx+1) "" in
  let y_coord = match int_of_string_opt y_coord with
  | None -> failwith "Invalid y coordinate"
  | Some num -> num in
  (x_coord, y_coord)

let perform_all lines = 
  let lights = Array.make_matrix 1000 1000 false in
  for i = 0 to (List.length lines - 1) do
    let line = List.nth lines i in
    let action = action_from_string line in
    let idx_first_comma = match String.index_opt line ',' with
    | None -> failwith "Invalid input"
    | Some idx -> idx in
    let idx_second_comma = match String.index_from_opt line (idx_first_comma + 1) ',' with
    | None -> failwith "Invalid input"
    | Some idx -> idx in
    let first_coord = get_coord line idx_first_comma in
    let second_coord = get_coord line idx_second_comma in
    perform action first_coord second_coord lights
  done;
  lights

let change_all_brightnesses lines =
  let lights = Array.make_matrix 1000 1000 0 in
  for i = 0 to (List.length lines - 1) do
    let line = List.nth lines i in
    let action = action_from_string line in
    let idx_first_comma = match String.index_opt line ',' with
    | None -> failwith "Invalid input"
    | Some idx -> idx in
    let idx_second_comma = match String.index_from_opt line (idx_first_comma + 1) ',' with
    | None -> failwith "Invalid input"
    | Some idx -> idx in
    let first_coord = get_coord line idx_first_comma in
    let second_coord = get_coord line idx_second_comma in
    change_brightness action first_coord second_coord lights
  done;
  lights

let rec count_on_lights (i,j) lights total =
  if j > (Array.length lights - 1) then total else
  if i > (Array.length lights.(j) - 1) then
    count_on_lights (0,j+1) lights total
  else
    let total = match lights.(i).(j) with
    | true -> total + 1
    | false -> total in
    count_on_lights (i+1,j) lights total

let rec count_total_brightness (i,j) lights total =
  if j > (Array.length lights - 1) then total else
  if i > (Array.length lights.(j) - 1) then
    count_total_brightness (0,j+1) lights total
  else
    let total = total + lights.(i).(j) in
    count_total_brightness (i+1,j) lights total

let p1 lines = 
  let lights = perform_all lines in
  count_on_lights (0,0) lights 0

let p2 lines =
  let lights = change_all_brightnesses lines in
  count_total_brightness (0,0) lights 0

let () =
  let lines = read_lines filename in
  let p1 = p1 lines in
  Printf.printf "The answer for part 1 is: %d" p1;

  let p2 = p2 lines in
  Printf.printf "The answer for part 2 is: %d" p2;

  ()
