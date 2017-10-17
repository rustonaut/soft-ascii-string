
#[doc(hidden)]
#[macro_export]
macro_rules! assert_ok {
    ($e:expr) => ({
        match $e {
            Ok(res) => res,
            Err(err) => panic!("expected Ok(..) got Err({:?})", err)
        }
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_err {
    ($e:expr) => ({
        match $e {
            Ok(badres) => panic!("expected Err(..) got Ok({:?})", badres),
            Err(err) => err
        }
    });
}