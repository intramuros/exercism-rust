#[macro_export]
macro_rules! hashmap {
    ($ ($key:expr => $value:expr),*) => {
        {
            let mut h_map = ::std::collections::HashMap::new();
            $(h_map.insert($key, $value);)*
            h_map
        }
    };
    ($ ($key:expr => $value:expr),+,) => {
        hashmap![$($key => $value),*]
    };
}
