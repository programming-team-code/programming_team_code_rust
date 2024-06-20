//! # Unsafe Recursive Closure which can mutate vars in surrounding scope

/// - see <https://github.com/EgorKulikov/rust_algo/blob/master/algo_lib/src/misc/recursive_function.rs>
/// - see <https://github.com/SrTobi/fix_fn>
///
/// # Example
/// ```
/// use programming_team_code_rust::unsafe_recursive_closure;
///
/// // move closures are okay
/// let mut fib = unsafe_recursive_closure!(move |fib, i: u32| -> u32 {
///    if i <= 1 {
///       i
///    } else {
///       fib(i - 1) + fib(i - 2)
///    }
/// });
/// assert_eq!(fib(7), 13);
///
/// // closures with no return type are okay
/// let n = 5;
/// let mut adj = vec![vec![]; n];
/// for i in 1..n {
///    adj[i / 2].push(i);
///    adj[i].push(i / 2);
/// }
/// assert_eq!(2 * (n - 1), adj.iter().map(|elem| elem.len()).sum());
/// let mut dfs = unsafe_recursive_closure!(|dfs, u: usize, p: Option<usize>| {
///    adj[u].retain(|&v| Some(v) != p);
///    for &v in &adj[u] {
///       dfs(v, Some(u));
///    }
/// });
/// dfs(0, None);
/// assert_eq!(n - 1, adj.iter().map(|elem| elem.len()).sum());
/// ```
#[macro_export]
macro_rules! unsafe_recursive_closure {
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
