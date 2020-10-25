use inline_proc::inline_proc;
use paste::paste;
use std::iter::Map;
use std::ops::{Add, Div, Mul, Sub};

pub mod prelude {
    pub use super::{add, div, eval, mul, sub, Node};
}

/// A Lisp-ish node that evaluates to some value.
pub trait Node {
    type Return;
    fn eval(self) -> Self::Return;
}

/// Evaluate a Lisp expression oh dear lord whhhhyyyyy
pub fn eval<N, R>(n: N) -> R
where
    N: Node<Return = R>,
{
    n.eval()
}

/// Define primitive types as identity nodes.
macro_rules! identity_node {
    ( $($t:ty),* ) => {
        $(
            paste! {
                #[doc = "Implement identity evaluation for `" $t "`."]
                impl Node for $t {
                    type Return = $t;
                    fn eval(self) -> Self::Return {
                        self
                    }
                }
            }
        )*
    }
}

identity_node!(char, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, String);

pub fn add<A, B, C>(a: A, b: B) -> C
where
    A: Add<B, Output = C>,
{
    a + b
}

pub fn sub<A, B, C>(a: A, b: B) -> C
where
    A: Sub<B, Output = C>,
{
    a - b
}

pub fn mul<A, B, C>(a: A, b: B) -> C
where
    A: Mul<B, Output = C>,
{
    a * b
}

pub fn div<A, B, C>(a: A, b: B) -> C
where
    A: Div<B, Output = C>,
{
    a / b
}

impl<A, R> Node for Box<dyn Fn(A) -> R> {
    type Return = Self;
    fn eval(self) -> Self::Return {
        self
    }
}

impl<T> Node for Box<T> {
    type Return = T;
    fn eval(self) -> Self::Return {
        *self
    }
}

impl<A, R> Node for &'static mut dyn Fn(A) -> R {
    type Return = Self;
    fn eval(self) -> Self::Return {
        self
    }
}

impl<T> Node for &'static mut T {
    type Return = T;
    fn eval(self) -> Self::Return {
        *self
    }
}

pub fn map<I, E, F, R>(f: F, i: I) -> Map<I::IntoIter, F>
where
    F: FnMut(E) -> R,
    I: IntoIterator<Item = E>,
{
    i.into_iter().map(f)
}

pub fn reduce<I, E, F, R>(init: R, f: F, i: I) -> R
where
    F: FnMut(R, E) -> R,
    I: IntoIterator<Item = E>,
{
    i.into_iter().fold(init, f)
}

pub fn to_vec<T, I>(i: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    i.into_iter().collect()
}

macro_rules! impl_lisp_fn_node {
    () => {
        /// Implements evaluation for functions that take no arguments.
        impl<F, R> Node for (F,)
        where
            F: Fn() -> R,
        {
            type Return = R;
            fn eval(self) -> Self::Return {
                self.0()
            }
        }
    };

    ($($postfix:literal),+ $(,)?) => {
        paste! {
            /// Implement evaluation for functions that take 1 argument(s).
            impl<
                F,
                $([<Arg $postfix>],)+
                $([<Ret $postfix>],)+
                R,
            > Node for (F, $([<Arg $postfix>],)+)
            where
                F: Fn($([<Ret $postfix>],)+) -> R,
                $([<Arg $postfix>]: Node<Return = [<Ret $postfix>]>,)+
            {
                type Return = R;
                fn eval(self) -> Self::Return {
                    self.0(
                        $(self.$postfix.eval(),)+
                    )
                }
            }
        }
    };
}

#[inline_proc]
mod gen_lisp_fn_node_impls_proc_macro {
    metadata::ron!(
        edition: "2018",
        clippy: true,
        dependencies: {},
        exports: (
            bang_macros: {
                "gen_lisp_fn_node_impls": "gen_lisp_fn_node_impls",
            }
        )
    );

    use super::*;
    use proc_macro::TokenStream;

    const MAX_ARGS: usize = 30;

    pub fn gen_lisp_fn_node_impls(_input: TokenStream) -> TokenStream {
        let mut macro_calls: Vec<String> = Vec::with_capacity(MAX_ARGS + 1);
        macro_calls.push("impl_lisp_fn_node!();".into());

        let mut args_string = String::from("1");

        for i in 1..=MAX_ARGS {
            macro_calls.push(format!("impl_lisp_fn_node!({});", args_string.clone()));
            args_string.push_str(format!(", {}", i + 1).as_str());
        }

        macro_calls.join("").parse().unwrap()
    }
}

gen_lisp_fn_node_impls!();
