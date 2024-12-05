let read_whole_file filename = 
  let ic = open_in filename in
  let len = in_channel_length ic in
  let str = really_input_string ic len in
  close_in ic;
  str

let rec read_lines_channel ic =
  let open In_channel in
  match input_line ic with
    | None -> []
    | Some line -> line :: read_lines_channel ic

let read_lines filename =
  let ic = open_in filename in
  read_lines_channel ic

let list_of_hash_keys ht = Hashtbl.fold (fun k _v acc -> k :: acc) ht []

let list_of_hash_vals ht = Hashtbl.fold (fun _k v acc -> v :: acc) ht []
