pub mod airplane;
pub mod combinators;
pub mod game_console;
pub mod io;
pub mod iter;
pub mod luggage;
pub mod passport;
pub mod password;
pub mod toboggan;

#[macro_export]
macro_rules! cartesian {
    ($iter:expr) => {
        $iter.into_iter()
    };
    ($iter1:expr, $iter2:expr) => {
        $crate::combinators::cartesian::Cartesian::new($iter1, $iter2)
    };
    ($iter1:expr, $iter2:expr, $($rest:expr),+) => {
        $crate::combinators::tuple_join::TupleJoin::new($crate::combinators::cartesian::Cartesian::new($iter1, cartesian!($iter2, $($rest),+)))
    };
}
