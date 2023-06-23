#[macro_export]
macro_rules! printerr {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[1;31mError:\x1B[0m {}", format_args!($($arg)*));
        std::process::exit(1);
    })
}

#[macro_export]
macro_rules! printerr_noexit {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[1;31mError:\x1B[0m {}", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! printwarn {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[1;33mWarning:\x1B[0m {}", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! printinfo {
    ($($arg:tt)*) => ({
        eprintln!("\x1B[1;34mInfo:\x1B[0m {}", format_args!($($arg)*));
    })
}