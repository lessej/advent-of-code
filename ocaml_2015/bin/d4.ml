let get_hex num key =
  let input = key ^ Int.to_string num in
  let hashed = Digest.MD5.string input in
  Digest.MD5.to_hex hashed

let rec inc_and_try curr key padding_len =
  let hex = get_hex curr key in
  let prefix = String.sub hex 0 padding_len in
  let desired = String.make padding_len '0' in
  if String.equal prefix desired then curr
  else inc_and_try (curr + 1) key padding_len

let () = 
  let p1 = inc_and_try 0 "iwrupvqb" 5 in
  Printf.printf "The answer for part 1 is %d\n" p1;
  let p2 = inc_and_try 0 "iwrupvqb" 6 in
  Printf.printf "The answer for part 2 is %d\n" p2;

  ()
