use termion::{color, cursor, clear};

use shiplift::errors::Error;
use shiplift::{Images, PullOptions, Containers, ContainerOptions};

use http::StatusCode;

pub fn abort(message: String) {
    println!("{}{}{}", color::Fg(color::Red), message, color::Fg(color::Reset));
    ::std::process::exit(1);
}

pub fn pullimg(images: &Images, name: &str, mut prepend: Option<&str>) {
    if prepend == None {prepend = Some("  ");}
    println!("{}{}?{} [{}]", prepend.unwrap(), color::Fg(color::Cyan), color::Fg(color::Reset), name);
    match images.get(name).inspect() {
        Ok(_) => print!("{}{}{}✔{}{}\r", cursor::Up(1), prepend.unwrap(),   color::Fg(color::Green), color::Fg(color::Reset), cursor::Down(1)),
        Err(e) => match e {
            Error::Fault {code, message} => match code {
                StatusCode::NOT_FOUND => {
                    print!("{}{}{}⟳{}{}\r", cursor::Up(1), prepend.unwrap(), color::Fg(color::Yellow), color::Fg(color::Reset), cursor::Down(1));

                    let image = images
                        .pull(&PullOptions::builder().image(name.clone()).build())
                        .unwrap();
                    for progress in image {
                        print!("\r    ↪ {}", progress["status"].as_str().unwrap());
                        if progress.get("progress") != None {
                            print!(" ➜ {}", progress["progress"].as_str().unwrap());
                        }
                        print!("{}", clear::AfterCursor);
                    }
                    print!("{}\r", clear::CurrentLine);
                    print!("{}{}{}✔{}{}\r", cursor::Up(1), prepend.unwrap(), color::Fg(color::Green), color::Fg(color::Reset), cursor::Down(1));
                },
                _ => abort(message)
            },
            _ => abort(format!("{}", e))
        }
    }
}

pub fn checkcontainer(containers: &Containers, options: &ContainerOptions, mut prepend: Option<&str>) {
    if prepend == None {prepend = Some("  ");}
    println!("{}{}?{}", prepend.unwrap(), color::Fg(color::Cyan), color::Fg(color::Reset));
    match containers.get(match &options.name {Some(name) => &name, None => ""}).inspect() {
        Ok(_) => print!("{}{}{}✔{}{}\r", cursor::Up(1), prepend.unwrap(), color::Fg(color::Green), color::Fg(color::Reset), cursor::Down(1)),
        Err(e) => match e {
            Error::Fault {code, message} => match code {
                StatusCode::NOT_FOUND => { /* Creating container */
                    match containers.create(options) {
                        Ok(_) => print!("{}{}{}✔{}{}\r", cursor::Up(1), prepend.unwrap(), color::Fg(color::Green), color::Fg(color::Reset), cursor::Down(1)),
                        Err(e) => match e {
                            Error::Fault {code, message} => match code {
                                StatusCode::CONFLICT => println!("Okay"),
                                _ => abort(format!("{} -> {}: {}", code, message, options.serialize().unwrap()))
                            },
                            _ => abort(format!("{}", e))
                        }
                    }
                },
                _ => abort(message)
            },
            _ => abort(format!("{}", e))
        }
    }
}
