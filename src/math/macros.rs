/*
TODO
do we really want to use macros?
very flexible, but it reduces readability a lot
maybe we can use it for simple things and always have it well documented
*/

#[macro_export]
macro_rules! rug_int_vec {
    /*
    Allows us to write rug_int_vec![3, 2]
    instead of vec![Integer::from(3), [Integer::from(2)]
   */
    ($($x:expr),* $(,)?) => {
        vec![$(Integer::from($x)),*]
    };
}
