#[derive(Debug, PartialEq)]

enum Command {
    Play,
    Pause,
    Skip(u32),
    Resize { height: u32, width: u32 },
}

fn main() {
    let cmd: Command = Command::Pause;
    let cmd1: Command = Command::Skip(5);
    let cmd2: Command = Command::Resize {
        height: 10,
        width: 56,
    };

    println!("{:?}", cmd); // use derive(Debug) macro for printing the enum
    println!("{:?}", cmd1 == cmd2); // use derive(partialEq) for comparing to enum

    // Option<T> = Some(T) | None
    let x: Option<i32> = Some(1); // Option can be used to check presence of a value if it presnt -> Some() if not -> None.
    let x: Option<i32> = None; // Can be used in when accessing the elements of an array

    //Result<T, E> = OK(T) | Error(E)
    // "100" -> 100
    let x: Result<i32, String> = Ok(10);
    // "100hwssh" -> error
    let x: Result<i32, String> = Err("Error in parsing".to_string());
}
