use std::process;

#[inline]
pub fn unwrap_abort<T>( o: Option<T> ) -> T
{
    o.map_or_else(|| process::abort(), |t| t)
}

#[inline]
pub fn unwrap_r_abort<T, E>( r: Result<T, E> ) -> T
{
    r.map_or_else(|_| process::abort(), |t| t)
}
