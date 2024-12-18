val read_lines : string -> string list
val read_whole_file : string -> string
val list_of_hash_keys : ('a,'b) Hashtbl.t -> 'a list
val list_of_hash_vals : ('a,'b) Hashtbl.t -> 'b list
val list_of_hash : ('a,'b) Hashtbl.t -> ('a*'b) list
val insertion_sort : 'a list -> ('a -> 'a -> bool) -> 'a list
val permutations_of_list : 'a list -> 'a list list
val matrix_of_string_list : string list -> char array array
val print_char_char_matrix : char array array -> unit
