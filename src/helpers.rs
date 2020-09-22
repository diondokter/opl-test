#[macro_export]
macro_rules! debug_only {
    ($statement:stmt) => {
        if cfg!(debug_assertions) {
            $statement
        }
    };
    ($code:block) => {
        if cfg!(debug_assertions) {
            $code
        }
    };
}

#[macro_export]
macro_rules! debugger_only {
    ($statement:stmt) => {
        if stm32f4xx_hal::stm32::DCB::is_debugger_attached() {
            $statement
        }
    };
    ($code:block) => {
        if stm32f4xx_hal::stm32::DCB::is_debugger_attached() {
            $code
        }
    };
}

#[macro_export]
macro_rules! release_only {
    ($statement:stmt) => {
        if cfg!(not(debug_assertions)) {
            $statement
        }
    };
    ($code:block) => {
        if cfg!(not(debug_assertions)) {
            $code
        }
    };
}
