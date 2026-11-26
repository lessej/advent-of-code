open Ocaml_2015.Lib

let filename = "bin/input/d10.txt"

let expand seq =
  let rec group dig count next = function
    | [] -> 
        dig::(Char.chr (count + 48))::next
    | c::rest when c = dig ->
        group dig (count + 1) next rest
    | c::rest ->
        let next = dig::(Char.chr (count + 48))::next in
        group c 1 next rest
  in
  group (List.nth seq 0) 0 [] seq
    |> List.rev

let p1 start =
  let rec las seq = function
    | 0 -> seq
    | i ->
        let seq = expand seq in
        las seq (i - 1)
  in
  las start 40

let p2 start = 
  let rec las seq = function
    | 0 -> seq
    | i ->
        let seq = expand seq in
        las seq (i - 1)
  in
  las start 50

let () =
  let lines = read_lines filename in
  let line = List.nth lines 0
    |> String.to_seq
    |> List.of_seq
  in
  let p1_res = p1 line
    |> List.length
  in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 line
    |> List.length
  in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
