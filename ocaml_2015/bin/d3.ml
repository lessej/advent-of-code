type dir =  
  Up
  | Down
  | Left
  | Right;;

let dir_from_char d = 
  Printf.printf "Char: %s\n" (String.make 1 d);
  match d with
  | '>' -> Right
  | '<' -> Left
  | '^' -> Up
  | 'v' -> Down
  | _ -> failwith "Unexpected direction"

let get_new_pos (x, y) dir = match dir with
  | Up -> (x, y + 1)
  | Down -> (x, y - 1)
  | Left -> (x - 1, y)
  | Right -> (x + 1, y)

let read_whole_file =
  let ch = open_in "bin/input/d3.txt" in
  let n = in_channel_length ch in
  let s = really_input_string ch n in
  close_in ch;
  s

let rec calc_total_p1 input i (x, y) visited =
  if i >= String.length input - 1 then
    Hashtbl.length visited
  else 
    let hk = Int.to_string x ^ Int.to_string y in 
    let c = String.get input i in
    let new_dir = dir_from_char c in
    let new_pos = get_new_pos (x, y) new_dir in
    match Hashtbl.find_opt visited hk with
      | None ->
        Hashtbl.add visited hk true;
        calc_total_p1 input (i + 1) new_pos visited
      | Some _ -> calc_total_p1 input (i + 1) new_pos visited

let rec calc_total_p2 input i (x, y) visited =
  if i >= String.length input - 1 then
    Hashtbl.length visited
  else 
    let hk = Int.to_string x ^ Int.to_string y in 
    let c = String.get input i in
    let new_dir = dir_from_char c in
    let new_pos = get_new_pos (x, y) new_dir in
    match Hashtbl.find_opt visited hk with
      | None ->
        Hashtbl.add visited hk true;
        calc_total_p2 input (i + 2) new_pos visited
      | Some _ -> calc_total_p2 input (i + 2) new_pos visited

let () = 
  let contents = read_whole_file in
  let ht_p1 = Hashtbl.create 1000 in
  let total_p1 = calc_total_p1 contents 0 (0, 0) ht_p1 in
  Printf.printf "The total for p1 is: %d\n" total_p1;

  let ht_p2 = Hashtbl.create 1000 in
  let santa_p2 = calc_total_p2 contents 0 (0, 0) ht_p2 in
  Printf.printf "Santa: %d\n" santa_p2;
  let robo_p2 = calc_total_p2 contents 1 (0, 0) ht_p2 in
  Printf.printf "Robo: %d\n" robo_p2;
  Printf.printf "The total for p2 is: %d\n" robo_p2;
  ()
