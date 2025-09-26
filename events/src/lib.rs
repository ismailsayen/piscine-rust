use chrono::Duration;
use colored::*;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
	Top,
	Bottom,
	Center,
}

#[derive(Debug, Eq, PartialEq)]
 pub struct Notification {
	pub size: u32,
	pub color: (u8, u8, u8),
	pub position: Position,
	pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
	Remainder(&'a str),
	Registration(Duration),
	Appointment(&'a str),
	Holiday,
}
// (Bottom, 50, [38;2;50;50;50mGo to the doctor[0m)
use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
		write!(f,"{:?}, {}, {}",self.position,self.size,self.content.truecolor(self.color.0, self.color.1, self.color.2));
		Ok(())
	 }
}

use Event::*;

impl <'a> Event <'a> {
	pub fn notify(&self) -> Notification{
        match self {
			Remainder(s)=>{
				Notification{
					size:50,
					color: (50, 50, 50),
					position: Position::Bottom,
					content:s.to_string()
				}
			},
	 Registration(t) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {} left before the registration ends",
                    format_time(t)
                ),
            },
            Appointment(s) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: s.to_string(),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

fn format_time(time: &Duration) -> String {
    let mut seconds = time.num_seconds();
    let hours = seconds / 3600;
    seconds = seconds % 3600;
    let minute = seconds / 60;
    seconds = seconds % 60;

    format!("{}H:{}M:{}S", hours, minute, seconds)
}