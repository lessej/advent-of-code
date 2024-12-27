open Ocaml_2024.Lib

let filename = "input/d20.txt"

let neighbors = [|(-1,0);(0,-1);(1,0);(0,1)|]

let add_path_to_list matrix =
  let rec find_in x y c =
    if x >= Array.length matrix.(0) then find_in 0 (y+1) c else
    if y >= Array.length matrix then failwith "Didn't find start" else
    if matrix.(y).(x) = c then (x,y) else
    find_in (x+1) y c
  in
  let (sx,sy) = find_in 0 0 'S' in
  let (ex,ey) = find_in 0 0 'E' in
  let rec follow x y v path =
    if x = ex && y = ey then ((x,y)::path) else
    let (nx,ny) = Array.fold_left(fun acc (nx,ny) ->
      (x+nx,y+ny)::acc
    ) [] neighbors 
      |> List.filter (fun (nx,ny) ->
        (matrix.(ny).(nx) = '.' || matrix.(ny).(nx) = 'E') && 
        (Hashtbl.find_opt v (nx,ny) |> Option.is_none)
      ) 
      |> List.hd
    in
    Hashtbl.replace v (x,y) true;
    (x,y)::path |> follow nx ny v
  in
  let hm = Hashtbl.create 100 in
  let v = Hashtbl.create 100 in
  follow sx sy v []  
    |> List.iteri (fun i (x,y) ->
        Hashtbl.replace hm (x,y) i
    );
  hm

let find_cheats_with_dist path cl cd =
  let all_points = list_of_hash path in
  list_of_hash path
    |> List.fold_left (fun acc ((cx,cy),cidx) ->
        List.filter (fun ((nx,ny),nidx) ->
          let m_dist = (Int.abs (nx-cx)) + (Int.abs (ny-cy)) in
          let c_dist = cidx - nidx - m_dist in 
          m_dist <= cl && c_dist >= cd
        ) all_points
          |> List.length
          |> Int.add acc
    ) 0

let solve lines =
  let path = matrix_of_string_list lines |> add_path_to_list in 
  let p1_res = find_cheats_with_dist path 2 100 in
  let p2_res = find_cheats_with_dist path 20 100 in
  (p1_res,p2_res)

let () = 
  let lines = read_lines filename in
  let (p1_res,p2_res) = solve lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
