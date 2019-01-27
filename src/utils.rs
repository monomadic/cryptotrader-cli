#![allow(dead_code)]
#![allow(unused_variables)]

pub fn iter_map<T,F>(vec: Vec<T>, method: F) -> Vec<T>
where F:Fn(T) -> T {
	vec.into_iter()
		.map(method)
    	.collect()
}
