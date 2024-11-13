open Printf
open In_channel

let read_whole_chan chan =
  let rec loop curr_floor curr_pos =
    if curr_floor == -1 then 
      curr_pos 
    else
      let c = input_char chan in
      match c with
        | None -> curr_floor
        | Some '(' -> loop (curr_floor + 1) (curr_pos + 1)
        | Some ')' -> loop (curr_floor - 1) (curr_pos + 1)
        | Some _ -> loop (curr_floor) (curr_pos + 1)
  in
  loop 0 0

let read_whole_file filename =
  let chan = open_in filename in
    read_whole_chan chan

let main () =
  let filename = "bin/input/day_1.txt" in
  let final_floor = read_whole_file filename in
  printf "He ends up on floor number %d\n" final_floor

let () = main ();
