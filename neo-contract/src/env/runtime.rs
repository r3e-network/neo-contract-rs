/// Get the operation string from the current invocation
pub fn get_invocation_operation() -> ByteString {
    unsafe {
        extern "C" {
            fn neo_runtime_get_invocation_operation() -> *const u8;
            fn neo_runtime_get_invocation_operation_length() -> usize;
        }
        
        // Get operation string from runtime
        let ptr = neo_runtime_get_invocation_operation();
        let len = neo_runtime_get_invocation_operation_length();
        
        if ptr.is_null() || len == 0 {
            ByteString::default()
        } else {
            let slice = core::slice::from_raw_parts(ptr, len);
            ByteString::from(slice)
        }
    }
}

/// Get the arguments from the current invocation
pub fn get_invocation_args() -> Vec<Any> {
    unsafe {
        extern "C" {
            fn neo_runtime_get_invocation_args_count() -> usize;
            fn neo_runtime_get_invocation_arg(index: usize) -> *const u8;
            fn neo_runtime_get_invocation_arg_length(index: usize) -> usize;
        }
        
        // Get number of arguments
        let arg_count = neo_runtime_get_invocation_args_count();
        let mut args = Vec::with_capacity(arg_count);
        
        // Get each argument
        for i in 0..arg_count {
            let ptr = neo_runtime_get_invocation_arg(i);
            let len = neo_runtime_get_invocation_arg_length(i);
            
            if !ptr.is_null() && len > 0 {
                let slice = core::slice::from_raw_parts(ptr, len);
                let any_value = Any::deserialize(slice);
                args.push(any_value);
            } else {
                args.push(Any::null());
            }
        }
        
        args
    }
}

/// Set the result of the current invocation
pub fn set_invocation_result(result: Any) {
    unsafe {
        extern "C" {
            fn neo_runtime_set_invocation_result(ptr: *const u8, len: usize);
        }
        
        // Serialize the result to bytes
        let serialized = result.serialize();
        
        // Set result in runtime
        neo_runtime_set_invocation_result(serialized.as_ptr(), serialized.len());
    }
} 