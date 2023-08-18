use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::{thread, time::Duration};

struct Point {
    x: i32,
    y: i32,
}

struct Snake {
    body: Vec<Point>,
    direction: (i32, i32),
}

impl Snake {
    fn move_forward(&mut self) {
        let (dx, dy) = self.direction;
        let head: Point = self.body[0];
        let new_head: Point = Point {
            x: head.x + dx,
            y: head.y + dy,
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }

    // Add methods to handle direction changes and collisions
}

fn main() {
    let mut stdout: std::io::Stdout = std::io::stdout();
    terminal::enable_raw_mode().unwrap();

    let (width, height) = terminal::size().unwrap();
    let start_x: i32 = (width / 2).into();
    let start_y: i32 = (height / 2).into();

    let mut snake = Snake {
        body: vec![
            Point {
                x: start_x,
                y: start_y,
            },
            Point {
                x: start_x - 1,
                y: start_y,
            },
            Point {
                x: start_x - 2,
                y: start_y,
            },
        ],
        direction: (1, 0),
    };

    loop {
        stdout.queue(terminal::Clear(ClearType::All));
        for point in &snake.body {
            stdout
                .queue(cursor::MoveTo(point.x as u16, point.y as u16))
                .queue("■");
        }
        stdout.flush().unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent {
                code,
                modifiers,
                state,
                kind,
            }) = event::read().unwrap()
            {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => break,
                    KeyCode::Up if snake.direction != (0, 1) => snake.direction = (0, -1),
                    KeyCode::Down if snake.direction != (0, -1) => snake.direction = (0, 1),
                    KeyCode::Left if snake.direction != (1, 0) => snake.direction = (-1, 0),
                    KeyCode::Right if snake.direction != (-1, 0) => snake.direction = (1, 0),
                    _ => {}
                }
            }
        }

        snake.move_forward();
        thread::sleep(Duration::from_millis(100));
    }

    terminal::disable_raw_mode().unwrap();
}
