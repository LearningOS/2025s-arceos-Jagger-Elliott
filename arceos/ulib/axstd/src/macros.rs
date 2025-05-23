//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        // $crate::io::__print_impl(format_args!("\x1b[31m"));
        $crate::io::__print_impl(format_args!($($arg)*));
        // $crate::io::__print_impl(format_args!("\x1b[0m"));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        // $crate::io::__print_impl(format_args!("\x1b[31m"));
        // $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
        $crate::io::__print_impl(format_args!("\u{1B}[{}m{}\u{1B}[m\n", 32, format_args!($($arg)*)));
        // $crate::io::__print_impl(format_args!("\x1b[0m"));
    }
}
