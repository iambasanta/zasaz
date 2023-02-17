use std::io;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

fn main() {
    println!("Enter work time (minutes): ");
    let mut work = String::new();

    io::stdin()
        .read_line(&mut work)
        .expect("Failed to read work time!");

    let work:u32 = work.trim().parse().expect("Invalid input!");

    println!("Enter rest time (minutes): ");
    let mut rest = String::new();

    io::stdin()
        .read_line(&mut rest)
        .expect("Failed to read rest time!");

    let rest:u32 = rest.trim().parse().expect("Invalid input!");

    pomodoro(work, rest);
}

fn pomodoro(work: u32, rest: u32) {
    println!("Work");
    countdown(work);
    clear_screen();

    println!("Time's up! Take a break.");
    println!("Rest");
    countdown(rest);
}

fn countdown(duration: u32) {
    for i in 0..duration {
        println!("{} minutes remaining...", duration - i);
        sleep(Duration::from_secs(60));
    }
}

fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    io::stdout().flush().unwrap();
}
