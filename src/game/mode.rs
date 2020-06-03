use crate::GameResult;
use std::io::{Stdin, Stdout};

#[macro_export]
macro_rules! Menu {
    [$(
        #[field(display = $disp:literal, perform = $perf:expr)]
        $mode:ident
    ),+$(,)?] => {{
        let mut m = Menu::default();

        $(
            struct $mode;

            impl Mode for $mode {
                fn perform(&self, stdin: &Stdin, stdout: &mut Stdout) -> GameResult<()> {
                    $perf(stdin, stdout)
                }

                fn desc(&self) -> &'static str {
                    $disp
                }
            };

            m.new_mode(&$mode);
        )+;

        m
    }};
}

pub trait Mode {
    fn perform(&self, stdin: &Stdin, stout: &mut Stdout) -> GameResult<()>;
    fn desc(&self) -> &'static str;
}
