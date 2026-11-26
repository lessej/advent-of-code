open Ocaml_2015.Lib

let filename = "bin/input/d11.txt"

let increment pass =
  let pass = List.rev pass in
  let rec inc = function
    | [] -> []
    | 'z'::rest -> 'a'::(inc rest)
    | c::rest -> (Char.chr (Char.code c + 1))::rest
  in
  inc pass 
    |> List.rev

let is_asc a b c = (Char.code a + 1) = Char.code b && (Char.code b + 1) = Char.code c

type rep_rules = {
  ct: int;
  c: int;
}

let has_rep rep = rep.ct > 1

let check_pass pass =
  let rec check asc prev rep = function
    | [] | [_] -> 
        asc && has_rep rep
    | 'i'::_ | 'o'::_ | 'l'::_ -> false
    | l::r::rest ->
        let rep = if l = r then
          let ct = if Char.code l <> rep.c then rep.ct + 1 else rep.ct in
          { ct = ct; c = Char.code l; }
        else
          rep 
        in
        let asc = asc || is_asc prev l r in
        check asc l rep (r::rest)
  in
  check false 'z' { ct = 0; c = 27; } pass

let rec p1 pass =
  let incd = increment pass in
  if check_pass incd then 
    List.to_seq incd
      |> String.of_seq
  else p1 incd

let () =
  let lines = read_lines filename in
  let p1_pass = List.hd lines
    |> String.to_seq
    |> List.of_seq
  in
  let p1_res = p1 p1_pass in
  Printf.printf "The answer for part 1 is: %s\n" p1_res;
  let p2_res = String.to_seq p1_res |> List.of_seq |> p1 in
  Printf.printf "The answer for part 2 is: %s\n" p2_res;
  ()
