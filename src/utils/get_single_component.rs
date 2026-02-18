#[macro_export]
macro_rules! get_single_component {
    ($var:ident) => {
        match $var.single() {
            Ok(v) => v,
            Err(_) => {
                println!("No {} found", stringify!($var));
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! get_single_mut_component {
    ($var:ident) => {
        match $var.single_mut() {
            Ok(v) => v,
            Err(_) => {
                println!("No {} found", stringify!($var));
                return;
            }
        }
    };
}
