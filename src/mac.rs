#[macro_export]
macro_rules! muncher {
    (@internal $input:ident $from:ident let $pat:pat = $expr:expr, $($tt:tt)+) => {{
        let $pat = $expr;
        muncher!(@internal $input $from $($tt)+)
    }};
    (@internal $input:ident $from:ident ($expr:expr)) => {
        match $expr {
            Ok(output) => Ok(($from, output)),
            Err(error) => Err(($from, error)),
        }
    };
    (@internal $input:ident $from:ident $ident:ident <- @match ($expr:expr) {
        $($pat:pat => $parser:expr,)+
    }, $($tt:tt)+) => {{
        let ($from, $ident) = match $expr {
            $($pat => $crate::Parser::parse(&mut $parser, $input, $from)?,)+
        };
        muncher!(@internal $input $from $($tt)+)
    }};
    (@internal $input:ident $from:ident $ident:ident <- $parser:expr, $($tt:tt)+) => {{
        let ($from, $ident) = $crate::Parser::parse(&mut $parser, $input, $from)?;
        muncher!(@internal $input $from $($tt)+)
    }};
    (@internal $input:ident $from:ident $parser:expr, $($tt:tt)+) => {{
        let ($from, _) = $crate::Parser::parse(&mut $parser, $input, $from)?;
        muncher!(@internal $input $from $($tt)+)
    }};
    ($($tt:tt)+) => {
        $crate::P(|input, from| muncher!(@internal input from $($tt)+))
    }
}
