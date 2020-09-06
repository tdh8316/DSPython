#[macro_export]
macro_rules! err {
    ($self:ident, $e:expr, $desc:expr) => {{
        Err(LLVMCompileError::new(
            Some($self.get_loc()),
            $e($desc.to_string()),
        ))
    }};
    ($self:ident, $e:expr, $s1:expr, $s2:expr) => {{
        Err(LLVMCompileError::new(
            Some($self.get_loc()),
            $e($s1.to_string(), $s2.to_string()),
        ))
    }};
}
