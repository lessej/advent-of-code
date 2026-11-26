open Ocaml_2015.Lib

let filename = "bin/input/d9.txt"

(*
Distances:
  (start, end): dist
  (end, start): dist

Locations:
  one set

1. Find all permutations
2. Trace each pairwise and get distances
3. Take shortest
*)

module PairMap = Map.Make(struct
  type t = string * string
  let compare = compare
end)

module StringSet = Set.Make(String)

let normalize (a,b) =
  if String.compare a b <= 0 then (a,b)
  else (b,a)

let parse_lines lines =
  let rec parse res = function
    | [] -> res
    | curr::rest ->
        let (start, fin, dist) = String.split_on_char ' ' curr
          |> List.map (fun s -> String.trim s)
          |> (fun parts -> (List.nth parts 0, List.nth parts 2, int_of_string (List.nth parts 4))) in
        parse (res @ [(start, fin, dist)]) rest
  in
  parse [] lines

let cities_map lines =
  let map = PairMap.empty in
  let rec add res = function
    | [] -> res
    | (start, fin, dist)::rest ->
        let (start, fin) = normalize (start, fin) in
        let res = PairMap.add (start, fin) dist res in
        add res rest
  in
  add map lines

let cities_set lines =
  let set = StringSet.empty in
  let rec add res = function
    | [] -> res
    | (start, fin, _dist)::rest ->
        let res = StringSet.add start res in
        let res = StringSet.add fin res in
        add res rest
  in
  add set lines

let rec permutations = function 
  | [] -> [[]]
  | xs ->
      List.concat_map
      (fun x -> 
        let rest = List.filter ((<>) x) xs in
        List.map (fun p -> x :: p) (permutations rest))
      xs

let line_total dists perm =
  let rec pairwise total = function
    | [] | [_] -> total
    | prev::next::rest ->
        let (l, r) = normalize (prev, next) in
        let dist = PairMap.find (l, r) dists in
        pairwise (total + dist) (next::rest)
  in
  pairwise 0 perm

let find_mm dists op perms =
  match perms with
  | [] -> failwith "missing permutations"
  | curr::rest ->
      List.fold_left
        (fun acc perm -> op acc (line_total dists perm))
        (line_total dists curr)
        rest

let p1 lines =
  let lines = parse_lines lines in
  let cities = cities_map lines in
  cities_set lines
    |> StringSet.elements
    |> permutations
    |> find_mm cities min

let p2 lines =
  let lines = parse_lines lines in
  let cities = cities_map lines in
  cities_set lines
    |> StringSet.elements
    |> permutations
    |> find_mm cities max

let () =
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
