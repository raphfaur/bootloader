use numtoa;

#[macro_export]
macro_rules! debug {
    ($a : expr) => {
        let mut buffer = [0u8; 20];
        print_str(stringify!($a));
        print_str(" : ");
        print_str($a.numtoa_str(16, &mut buffer));
        printc(10);
        printc(13);
    };
}

pub use debug;