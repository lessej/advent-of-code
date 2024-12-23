open Ocaml_2024.Lib

let filename = "input/d19.txt"

let parse lines =
  let parse_avail line =
    String.split_on_char ',' line |> List.map (fun s -> String.trim s) |> List.rev
  in
  let avail = List.nth lines 0 in
  let arrangements = List.filteri (fun i _ -> i > 1) lines in
  let avail = parse_avail avail in
  (avail, arrangements)

let hashmap_of_prefix_char avail =
  let hm = Hashtbl.create 100 in
  let rec add_to arrs hm =
    match arrs with
    | [] -> hm
    | curr::rest ->
        let prefix = curr.[0] in
        let starts_with = match Hashtbl.find_opt hm prefix with
          | None -> curr::[]
          | Some lst -> curr::lst
        in
        Hashtbl.replace hm prefix starts_with;
        add_to rest hm
  in
  add_to avail hm

let check_match a b s e =
  let rec check j =
    if j >= e then true else
    if a.[j] != b.[j] then false else
    check (j+1)
  in
  check s

let try_arrange_all sol avail seen =
  let rec arrange s e curr_res =
    if e > String.length sol || not (check_match sol curr_res s e) then 0 else
    let remaining = String.sub sol e (String.length sol - e) in
    match Hashtbl.find_opt seen remaining with
    | Some count -> count
    | None -> (
      if e = String.length sol then 1 else
      let rec try_starts_with sw inner_res =
        match sw with
        | [] -> inner_res
        | curr::rest ->
            let curr_len = String.length curr in
            let inner_res = inner_res + arrange e (e+curr_len) (curr_res^curr) in
            try_starts_with rest inner_res
      in
      let iter_res = match Hashtbl.find_opt avail (sol.[e]) with
        | None -> 0
        | Some found -> try_starts_with found 0
      in
      Hashtbl.replace seen remaining iter_res;
      iter_res
    )
  in
  arrange 0 0 ""

let solve lines =
  let (avail,arrangements) = parse lines in
  let hm = hashmap_of_prefix_char avail in
  let seen = Hashtbl.create 100 in
  let res = List.map (fun s ->
    try_arrange_all s hm seen
  ) arrangements
  in
  let p1_res = List.filter (fun r -> r > 0) res |> List.length in
  let p2_res = List.fold_left (fun acc r -> acc+r) 0 res in
  (p1_res,p2_res)

let () = 
  let lines = read_lines filename in
  let (p1_res,p2_res) = solve lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
