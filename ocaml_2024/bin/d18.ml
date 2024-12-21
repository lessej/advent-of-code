open Ocaml_2024.Lib

let filename = "input/d18.txt"

let next_coords = [(-1,0); (1,0); (0,-1); (0,1)]

let parse_corrupted lines =
  let rec parse lines res =
    match lines with
    | [] -> res
    | curr::rest ->
        let parts = String.split_on_char ',' curr |> List.map (fun s -> int_of_string s) in
        (List.nth parts 0, List.nth parts 1)::res |> parse rest
  in
  parse lines [] |> List.rev

let into_hashmap corrupted =
  let hm = Hashtbl.create 1024 in
  let rec add_to lines hm =
    match lines with
    | [] -> hm
    | curr::rest ->
        Hashtbl.replace hm curr true;
        add_to rest hm
  in
  add_to corrupted hm

let traverse_mem w h corrupted =
  let oob x y = 
    x < 0 || x > w || y < 0 || y > h 
  in
  let rec traverse q v =
    if Queue.is_empty q then None else
    let (cx,cy,path) = Queue.pop q in
    if cx = w && cy = h then Some path else
    if Hashtbl.find_opt v (cx,cy) |> Option.is_some then
      traverse q v
    else
      let () = Hashtbl.replace v (cx,cy) true in
      let () = List.map (fun (nx,ny) -> (cx+nx,cy+ny)) next_coords
        |> List.filter (fun (nx,ny) ->
            let is_oob = oob nx ny in
            let is_not_corrupted = Hashtbl.find_opt corrupted (nx,ny) 
              |> Option.is_none
            in
            not is_oob && is_not_corrupted
        )
        |> List.iter (fun (nx,ny) ->
            let new_path = (nx,ny)::path in
            Queue.push (nx,ny,new_path) q;
        )
      in
      traverse q v
  in
  let q = Queue.create () in
  let v = Hashtbl.create 1000 in
  Queue.push (0,0,[]) q;
  traverse q v

let p1 lines =
  let corrupted = parse_corrupted lines 
    |> List.filteri (fun i _ -> i < 1024)
  in
  let corrupted = into_hashmap corrupted in
  match traverse_mem 70 70 corrupted with
  | None -> failwith "couldn't reach the end"
  | Some x -> List.length x

let p2 lines =
  let corrupted_list = parse_corrupted lines in
  let corrupted = into_hashmap corrupted_list in
  let rec check_from_end corrupted_list =
    match corrupted_list with
    | [] -> failwith "Got to the end and didn't find the answer"
    | (cx,cy)::rest ->
        Hashtbl.remove corrupted (cx,cy);
        if traverse_mem 70 70 corrupted |> Option.is_some then
          (cx,cy)
        else
          check_from_end rest
  in
  List.rev corrupted_list |> check_from_end

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let (p2_res_x,p2_res_y) = p2 lines in
  Printf.printf "The answer for part 2 is: (%d,%d)\n" p2_res_x p2_res_y;
  ()
