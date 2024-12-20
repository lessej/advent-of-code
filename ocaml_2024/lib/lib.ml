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

let list_of_hash_keys ht = Hashtbl.fold (fun k _v acc -> k::acc) ht []

let list_of_hash_vals ht = Hashtbl.fold (fun _k v acc -> v::acc) ht []

let list_of_hash ht = Hashtbl.fold (fun k v acc -> (k,v)::acc) ht []

let insertion_sort unsorted f =
  let rec insert x l =
    match l with
    | [] -> [x]
    | y::ys -> if f x y then x::y::ys else y::insert x ys
  in
  let rec sort l =
    match l with
    | [] -> []
    | x::xs -> insert x (sort xs)
  in
  sort unsorted

let rec insert x perms_list =
  match perms_list with
  | [] -> [[x]]
  | h::t -> 
    (x::perms_list) :: (List.map (fun el -> h::el) (insert x t));;

let rec permutations_of_list input_list =
  match input_list with
  | [] -> [input_list]
  | h::t -> 
    List.flatten (List.map (insert h) (permutations_of_list t));;

let matrix_of_string_list lines =
  let rec matrix_line rem arr_list =
    match rem with
    | [] -> arr_list
    | h::t ->
      let char_arary = Array.init (String.length h) (fun i -> h.[i]) in
      matrix_line t (char_arary :: arr_list)
  in
  Array.of_list (matrix_line (List.rev lines) [])

let print_char_char_matrix matrix =
  Array.iter (fun line ->
    Array.iter (fun c ->
      Printf.printf "%c" c;
    ) line;
    Printf.printf "\n";
  ) matrix

let rec pow a = function
  | 0 -> 1
  | 1 -> a
  | n -> 
    let b = pow a (n / 2) in
    b * b * (if n mod 2 = 0 then 1 else a)
