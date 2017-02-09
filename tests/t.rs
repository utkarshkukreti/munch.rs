#[macro_export]
macro_rules! t {
    ($($parser:expr => {
        $($input:expr => $result:expr,)*
    },)*) => {{
        $($({
            assert_eq!(munch::Parser::parse(&mut $parser, $input, 0), $result);
            assert_eq!(match munch::Parser::parse(&mut $parser, concat!("🐱", $input), 4) {
                Ok((from, output)) => Ok((from - 4, output)),
                Err((from, error)) => Err((from - 4, error)),
            }, $result);
        })*)*
    }}
}
