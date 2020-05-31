#[macro_export]
macro_rules! Menu {
    ($(($disp:literal, $mode:ident, $perf:expr)),+) => {{
        let mut m: Menu = Default::default();
        $(
            struct $mode;

            impl<'a> Mode<'a> for $mode {
                fn perform(&self) {
                    $perf();
                }

                fn desc(&self) -> &'a str {
                    $disp
                }
            };

            m.new_mode(&$mode);
        )+;

        m
    }};
}

pub trait Mode<'a> {
    fn perform(&self);
    fn desc(&self) -> &'a str;
}
