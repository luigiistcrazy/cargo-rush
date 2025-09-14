#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        use crossterm::style::Stylize;
        println!("{} \"{}\"", "[Debug]:".bold().dim(), $msg);
    };
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        println!("{} \"{}\"", "[Info]:".bold().grey(), $msg);
    };
}

#[macro_export]
macro_rules! warning {
    ($msg:expr) => {
        println!("{} \"{}\"", "[Warning]:".bold().yellow(), $msg);
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        println!("{} \"{}\"", "[Error]:".bold().red(), $msg);
    };
}
