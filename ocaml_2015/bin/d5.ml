open Ocaml_2015.Lib

let filename = "bin/input/d5.txt";;

let check_vowels_with_repeat vowels c =
  match c with 
    | 'a' | 'e' | 'i' | 'o' | 'u' -> c :: vowels
    | _ -> vowels

let check_vowels vowels c =
  match c with 
    | 'a' | 'e' | 'i' | 'o' | 'u' -> 
      (match List.find_opt (fun x -> x == c) vowels with
        | None -> c :: vowels
        | Some _ -> vowels)
    | _ -> vowels

let rec is_nice line (vowels, repeated) i =
  if i > (String.length line - 1) then 
    repeated && (List.length vowels >= 3)
  else 
    let repeated = match repeated with
      | true -> true
      | false -> line.[i] == line.[i -1] in
    let vowels = check_vowels_with_repeat vowels line.[i] in
    let two_char = String.sub line (i-1) 2 in
    let dirty = match two_char with
      | "ab" | "cd" | "pq" | "xy" -> true
      | _ -> false in

    if dirty then false 
    else is_nice line (vowels, repeated) (i + 1)

let rec count_nice count i (lines: string list) = 
  if i > (List.length lines - 1) then count else
  let line = List.nth lines i in
  let vowels = check_vowels [] line.[0] in
  let count = match (is_nice line (vowels, false) 1) with
    | true -> count + 1
    | false -> count in
  count_nice count (i + 1) lines


let check_palindrome line i =
  line.[i-2] == line.[i]

let rec is_new_nice line i hash (ok_repeat, ok_palin) = 
  if i > String.length line - 1 then false
  else
    let ok_palin = match ok_palin with 
      | true -> ok_palin
      | false -> check_palindrome line i in
    let check_repeat = String.sub line (i-1) 2 in
    let ok_repeat = match ok_repeat with
      | true -> ok_repeat
      | false ->
        (match Hashtbl.find_opt hash check_repeat with
          | None -> 
            let () = Hashtbl.add hash check_repeat (i-1,i) in
            false
          | Some pair -> 
            let (_, end_idx )= pair in
            (end_idx != (i-1))
        ) in
    match ok_palin && ok_repeat with
      | true -> true
      | false -> is_new_nice line (i+1) hash (ok_repeat, ok_palin)

let rec count_new_nice count i (lines: string list) =
  if i > (List.length lines - 1) then count else
  let line = List.nth lines i in
  let hash = Hashtbl.create 100 in
  let first_key = String.sub line 0 2 in
  let () = Hashtbl.add hash first_key (0,1) in
  let count = match is_new_nice line 2 hash (false, false) with
    | false -> count
    | true -> count + 1 in
  count_new_nice count (i+1) lines

let () = 
  let lines = read_lines filename in
  let total_nice = count_nice 0 0 lines in
  Printf.printf "The solution for part 1 is: %d\n" total_nice;
  let total_new_nice = count_new_nice 0 0 lines in
  Printf.printf "The solution for part 2 is: %d\n" total_new_nice;

  ()
