use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use anyhow::Result;

fn ask_input() -> String {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the user inputs.");

    input.trim().into()
}

fn main() -> Result<()> {
    let email = Message::builder()
        .from("No Coffee Craft<nocoffeecraft@gmail.com>".parse()?)
        .to("Abinash Sahoo<abinashsahoo1012@gmail.com>".parse()?)
        .subject("Happy new year")
        .body(String::from("Be happy!"))?;

    println!("Enter your email ID: ");
    let username = ask_input();

    println!("Enter your password: ");
    let password = ask_input();

    let creds = Credentials::new(username.to_owned(), password.to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }

    Ok(())
}
