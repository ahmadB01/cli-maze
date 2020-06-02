use crate::GameResult;

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
                fn perform(&self) -> GameResult<()> {
                    $perf()
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
    fn perform(&self) -> GameResult<()>;
    fn desc(&self) -> &'static str;
}
