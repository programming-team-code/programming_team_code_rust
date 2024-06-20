//! # Recursive Closure which can mutate vars in surrounding scope

/// - see https://github.com/EgorKulikov/rust_algo/blob/master/algo_lib/src/misc/recursive_function.rs
/// - see https://github.com/SrTobi/fix_fn
///
/// # Example
/// ```
/// use programming_team_code_rust::helpers::recursive_closure::recursive_closure;
/// //use helpers::recursive_closure::recursive_closure;
///
/// let fib = recursive_closure!(|fib, i: u32| -> u32 {
///    if i <= 1 {
///       i
///    } else {
///       // fib will call the closure recursively
///       fib(i - 1) + fib(i - 2)
///    }
/// });
///
/// assert_eq!(fib(7), 13);
/// ```
#[macro_export]
macro_rules! recursive_closure {
    (
        $($mov:ident)? |$self_arg:ident $(, $arg_name:ident : $arg_type:ty)* $(,)? | $(-> $ret_type:ty)?
        $body:block
    ) => {{

        trait HideFn {
            fn call(&mut self, $($arg_name : $arg_type ,)*) $(-> $ret_type)?;
        }

        struct HideFnImpl<F: FnMut(&mut dyn HideFn, $($arg_type ,)*) $(-> $ret_type)?>(
            std::cell::UnsafeCell<F>,
        );

        impl<F> HideFnImpl<F>
        where
            F: FnMut(&mut dyn HideFn, $($arg_type ,)*) $(-> $ret_type)?,
        {
            fn new(f: F) -> Self {
                Self {
                    0: std::cell::UnsafeCell::new(f),
                }
            }
        }

        impl<F: FnMut(&mut dyn HideFn, $($arg_type ,)*) $(-> $ret_type)?> HideFn for HideFnImpl<F> {
            #[inline]
            fn call(&mut self, $($arg_name : $arg_type ,)*) $(-> $ret_type)? {
                unsafe { (*self.0.get())(self, $($arg_name ,)*) }
            }
        }

        let mut inner = HideFnImpl::new(
            #[inline]
            $($mov)?
            |$self_arg, $($arg_name : $arg_type ,)*| $(-> $ret_type)? {
                let mut $self_arg = |$($arg_name : $arg_type ),*| $self_arg.call($($arg_name ,)*);
                {
                    $body
                }
            }
        );

        #[inline]
        move |$($arg_name : $arg_type),*| $(-> $ret_type)? {
            inner.call($($arg_name),*)
        }
    }};
    (
        $($mov:ident)? |$self_arg:ident : $self_type:ty $(, $arg_name:ident $(: $arg_type:ty)?)* $(,)? | $(-> $ret_type:ty)?
        $body:block
    ) => {
        compile_error!(concat!("First parameter ", stringify!($self_arg), " may not have type annotation!"));
    };
    (
        $($mov:ident)? |$self_arg:ident $(, $arg_name:ident $(: $arg_type:ty)?)* $(,)? | $(-> $ret_type:ty)?
        $body:block
    ) => {
        compile_error!("All parameters except first need to have an explicit type annotation!");
    };
}
pub(crate) use recursive_closure;
