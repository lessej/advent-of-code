open Ocaml_2024.Lib

let filename = "input/d16.txt"

type dir =
  | North
  | South
  | East
  | West

module E = struct
  type t = (int * int * int * dir * (int * int) list)
  let compare a b =
    let (_,_,a,_,_) = a in
    let (_,_,b,_,_) = b in
    Stdlib.compare a b
end
module PQ = Binary_heap.Make(E)

let get_lr d =
  match d with
  | North -> (West,East)
  | South -> (East,West)
  | East -> (North,South)
  | West -> (South,North)

let get_in_grid grid c =
  let rec search x y =
    if x >= Array.length grid.(0) then
      search 0 (y+1)
    else
      if y >= Array.length grid then (-1,-1) else
      if grid.(y).(x) = c then (x,y) else
      search (x+1) y
  in
  search 0 0

let solve (sx:int) (sy:int) tx ty grid =
  let get_forward x y dir =
    let (x,y) = match dir with
      | North -> (x,y-1)
      | South -> (x,y+1)
      | East -> (x+1,y)
      | West -> (x-1,y)
    in
    if x >= 0 && x < Array.length grid.(0) && y >= 0 && y < Array.length grid && grid.(y).(x) != '#' then
      Some (x,y)
    else 
      None
  in
  let rec dijkstras vis queue bests =
    if PQ.is_empty queue then bests else
    let (cx,cy,dist,dir,path) = PQ.pop_minimum queue in
    let bests = if cx = tx && cy = ty then 
      (dist,path)::bests else bests 
    in
    if 
      Hashtbl.find_opt vis (cx,cy,dir) |> Option.is_some &&
      dist > Hashtbl.find vis (cx,cy,dir)
    then
      dijkstras vis queue bests
    else
      let () = Hashtbl.replace vis (cx,cy,dir) dist in
      let forward = get_forward cx cy dir in
      let () = if Option.is_some forward then
        let (fx,fy) = Option.get forward in
        let path = (fx,fy)::path in
        PQ.add queue (fx,fy,dist+1,dir,path)
      in
      let (l,r) = get_lr dir in
      PQ.add queue (cx,cy,dist+1000,l,path);
      PQ.add queue (cx,cy,dist+1000,r,path);
      dijkstras vis queue bests
  in
  let vis = Hashtbl.create 1000 in
  let queue = PQ.create ~dummy:(0,0,0,East,[]) 100 in
  PQ.add queue (sx,sy,0,East,((sx,sy)::[]));
  let bests = dijkstras vis queue [] in
  let best = List.fold_left (fun acc (x,_) -> 
    if x < acc then x else acc
  ) Int.max_int bests
  in
  let bs = Hashtbl.create 100 in
  List.filter (fun (dist,_) -> dist = best) bests
    |> List.iter (fun (_dist,path) ->
        List.iter (fun (x,y) ->
          Hashtbl.replace bs (x,y) true;
        ) path;
    );
  let seats = Hashtbl.length bs in
  (best,seats)

let p1p2 lines =
  let maze = matrix_of_string_list lines in
  let (sx,sy) = get_in_grid maze 'S' in
  let (tx,ty) = get_in_grid maze 'E' in
  solve sx sy tx ty maze

let () = 
  let lines = read_lines filename in
  let (p1_res,p2_res) = p1p2 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
