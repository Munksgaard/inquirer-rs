use std::io;

quick_error! {
    /// Inquirer Error
    #[derive(Debug)]
    pub enum Error {
        /// Error while dealing with file or stdin/stdout
        Io(err: io::Error) {
            from()
            cause(err)
            display("I/O error")
            description(err.description())
        }
        /// Invalid choice
        // TODO: Make this a type system error instead
        InvalidChoice(option_num: usize) {
            display("Option `{}` is not valid", option_num)
            description("Invalid choice")
        }
        /// No more input
        NoMoreInput {
            display("Didn't get any more input")
        }
        /// User pressed Ctrl-C
        UserAborted {
            display("User aborted (pressed Ctrl-C)")
        }
    }
}
