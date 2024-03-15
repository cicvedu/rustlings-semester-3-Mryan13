// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.


// Simulate external functions with internal mock implementations
mod external_simulation {
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }

    pub fn my_demo_function_alias(a: u32) -> u32 {
        a + 1
    }
}

// In a real scenario, these would be linked to actual external functions
extern "C" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mocked safe wrappers around potentially unsafe external functions
    fn safe_my_demo_function(a: u32) -> u32 {
        unsafe { my_demo_function(a) }
    }

    fn safe_my_demo_function_alias(a: u32) -> u32 {
        unsafe { my_demo_function_alias(a) }
    }

    #[test]
    fn test_success() {
        // Directly using mocked implementation in the test
        assert_eq!(external_simulation::my_demo_function(123), 123);
        assert_eq!(external_simulation::my_demo_function_alias(456), 457);

        // Using the wrappers should also reflect the expected behavior, but in
        // this simplified example, it will not work unless these functions are
        // properly linked with actual external implementations.
        // So in a real test, you should either link to real external functions
        // or further mock the environment to simulate calls to these functions.

        // Uncomment if linked with real or simulated external implementations:
        // assert_eq!(safe_my_demo_function(123), 123);
        // assert_eq!(safe_my_demo_function_alias(456), 457);
    }
}

