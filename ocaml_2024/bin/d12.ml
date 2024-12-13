open Ocaml_2024.Lib

let filename = "input/d12.txt"

let is_uppercase c =
  let ioc = int_of_char c in
  65 <= ioc && ioc <= 90

let get_raw_next_coords x y =
  (x-1,y)::(x,y-1)::(x+1,y)::(x,y+1)::[]

let fill_plot x y c matrix =
  let visited = Char.lowercase_ascii c in
  let out_of_bounds x y =
    x >= Array.length matrix.(0) || x < 0 || y >= Array.length matrix || y < 0
  in
  let check_matches nx ny =
    if out_of_bounds nx ny then false else
    matrix.(ny).(nx) = c || matrix.(ny).(nx) = visited
  in
  let get_same_around nx ny =
    let arr = Array.make 8 false in
    check_matches (nx-1) ny |> Array.set arr 0;
    check_matches (nx-1) (ny-1) |> Array.set arr 1;
    check_matches nx (ny-1) |> Array.set arr 2;
    check_matches (nx+1) (ny-1) |> Array.set arr 3;
    check_matches (nx+1) ny |> Array.set arr 4;
    check_matches (nx+1) (ny+1) |> Array.set arr 5; 
    check_matches nx (ny+1) |> Array.set arr 6; 
    check_matches (nx-1) (ny+1) |> Array.set arr 7;
    arr
  in
  let convex sa =
    let count = if not sa.(0) && not sa.(2) then 1 else 0 in
    let count = if not sa.(2) && not sa.(4) then count + 1 else count in
    let count = if not sa.(4) && not sa.(6) then count + 1 else count in
    let count = if not sa.(6) && not sa.(0) then count + 1 else count in
    count
  in
  let concave sa =
    let count = if sa.(0) && sa.(2) && not sa.(1) then 1 else 0 in
    let count = if sa.(2) && sa.(4) && not sa.(3) then count + 1 else count in
    let count = if sa.(4) && sa.(6) && not sa.(5) then count + 1 else count in
    let count = if sa.(6) && sa.(0) && not sa.(7) then count + 1 else count in
    count
  in
  let corner_count nx ny =
    let around = get_same_around nx ny in
    let convex = convex around in
    let concave = concave around in
    convex+concave
  in
  let get_next_coords x y =
    let check_coord nx ny =
      if out_of_bounds nx ny then None else
      match matrix.(ny).(nx) with
      | v when (v = visited || v = c) -> Some(nx,ny)
      | _ -> None
    in
    get_raw_next_coords x y |> List.fold_left (fun acc (nx,ny) ->
      match check_coord nx ny with
      | None -> acc
      | Some coord -> coord::acc
    ) [] 
  in
  let rec fill x y p a s =
    if (matrix.(y).(x) = visited) then (0,0,0) else
    let () = matrix.(y).(x) <- visited in
    let next_coords = get_next_coords x y in
    let (tap,taa,tas) = List.fold_left (fun (acc_p,acc_a,acc_s) (nx,ny) ->
      let (np,na,ns) = fill nx ny p a s in
      (acc_p+np,acc_a+na,acc_s+ns)
    ) (0,0,0) next_coords
    in
    let np = List.length next_coords |> Int.sub 4 |> Int.add tap in
    let cc = corner_count x y |> Int.add tas in
    (np,1+taa,cc)
  in
  fill x y 0 0 0 

let find_all matrix =
  let line_len = matrix.(0) |> Array.length in
  let lines_count = Array.length matrix in
  let rec find x y all =
    let y = if x >= line_len then y + 1 else y in
    let x = if x >= line_len then 0 else x in
    if y >= lines_count then all else
    let c = matrix.(y).(x) in
    if is_uppercase c |> not then
      (0,0,0)::all |> find (x+1) y
    else
      [fill_plot x y c matrix] |> List.append all |> find (x+1) y 
  in
  find 0 0 [] |> List.filter (fun li -> li != (0,0,0))

let p1 lines =
  matrix_of_string_list lines |> find_all |> List.fold_left (fun acc (p,a,_s) ->
    p*a |> Int.add acc
  ) 0

let p2 lines =
  matrix_of_string_list lines |> find_all |> List.fold_left (fun acc (_p,a,s) ->
    s*a |> Int.add acc
  ) 0

let () = 
  let lines = read_lines filename in
  let p1_res = p1 lines in
  Printf.printf "The answer for part 1 is: %d\n" p1_res;
  let p2_res = p2 lines in
  Printf.printf "The answer for part 2 is: %d\n" p2_res;
  ()
