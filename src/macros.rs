#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($msg:expr) => {{
        let label = <&str as crossterm::style::Stylize>::dim("[Debug]:").bold();
        println!("{} \"{}\"", label, $msg);
    }};
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {{
        let label = <&str as crossterm::style::Stylize>::grey("[Info]:").bold();
        println!("{} \"{}\"", label, $msg);
    }};
}

#[macro_export]
macro_rules! warning {
    ($msg:expr) => {{
        let label = <&str as crossterm::style::Stylize>::yellow("[Warning]:").bold();
        println!("{} \"{}\"", label, $msg);
    }};
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {{
        let label = <&str as crossterm::style::Stylize>::red("[Error]:").bold();
        println!("{} \"{}\"", label, $msg);
    }};
}
