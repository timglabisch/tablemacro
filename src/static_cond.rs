// copied from https://github.com/diesel-rs/diesel/blob/a4b8031f6a096c8ea4032e7d8f05711d7f0dea1a/diesel/src/macros/static_cond.rs

#[macro_export]
#[doc(hidden)]
macro_rules! static_cond {
    // private rule to define and call the local macro
    (@go $lhs:tt $rhs:tt $arm1:tt $arm2:tt) => {
        // note that the inner macro has no captures (it can't, because there's no way to escape `$`)
        macro_rules! __static_cond {
            ($lhs $lhs) => $arm1;
            ($lhs $rhs) => $arm2
        }

        __static_cond!($lhs $rhs);
    };

    // no else condition provided: fall through with empty else
    (if $lhs:tt == $rhs:tt $then:tt) => {
        $crate::static_cond!(if $lhs == $rhs $then else { });
    };
    (if $lhs:tt != $rhs:tt $then:tt) => {
        $crate::static_cond!(if $lhs != $rhs $then else { });
    };

    // we evaluate a conditional by generating a new macro (in an inner scope, so name shadowing is
    // not a big concern) and calling it
    (if $lhs:tt == $rhs:tt $then:tt else $els:tt) => {
        $crate::static_cond!(@go $lhs $rhs $then $els);
    };
    (if $lhs:tt != $rhs:tt $then:tt else $els:tt) => {
        $crate::static_cond!(@go $lhs $rhs $els $then);
    };
}