// This needs to be in its own file since we otherwise get a warning
// when blanket_impl is disabled (even when gated with #[cfg])
auto trait ForwardTryCloneToClone {}
