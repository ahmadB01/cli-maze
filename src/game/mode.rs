use crate::GameResult;

#[macro_export]
macro_rules! Menu {
    ($(($disp:literal, $Mode:ident, $perf:expr)),+) => {{
        let mut m: Menu = Default::default();
        $(
            struct $Mode;

            impl<'a> Mode<'a> for $Mode {
                fn perform(&self) -> GameResult<()> {
                    $perf()
                }

                fn desc(&self) -> &'a str {
                    $disp
                }
            };

            m.new_mode(&$Mode);
        )+;

        m
    }};
}

pub trait Mode<'a> {
    fn perform(&self) -> GameResult<()>;
    fn desc(&self) -> &'a str;
}
