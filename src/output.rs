use std::io::{Write, stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::vec::Vec;
use termion::raw::IntoRawMode;
use termion::color::{Rgb, Fg};

static TILES: [&str; 8] = ["\u{2581}", "\u{2582}",
                           "\u{2583}", "\u{2584}",
                           "\u{2585}", "\u{2586}",
                           "\u{2587}", "\u{2588}"];


#[derive(Copy, Clone)]
struct Tower {
    x: u16,
    heigth: u16,
}

impl Tower {                         
    pub fn get_heigth(&self) -> u16 {
        return self.heigth;
    }
                      
    pub fn print(&self, resolution: (u16, u16), stdout: &mut Stdout, color: Rgb) {
        for i in (resolution.1-(&self.heigth/8))..(resolution.1) {
            write!(stdout, "{}", termion::cursor::Goto(self.x+1, i+1)).unwrap();
            print!("{}{}", Fg(color), TILES[7]);
        }
        write!(stdout, "{}", termion::cursor::Goto(self.x+1, resolution.1-(&self.heigth/8))).unwrap();
        print!("{}{}", Fg(color), TILES[(&self.heigth%8) as usize]);
    }
          
    pub fn clear(&self, resolution: (u16, u16), stdout: &mut Stdout) {
        for i in (resolution.1-(&self.heigth/8)-1)..(resolution.1) {
            write!(stdout, "{}", termion::cursor::Goto(self.x+1, i+1)).unwrap();
            print!("\u{0020}");
        }
    }
}

struct Towers<'a> {
    tower_list: Vec<Tower>,
    resolution: (u16, u16),
    stdout: &'a mut Stdout,
}

impl<'a> Towers<'a> {
    pub fn new(stdout: &'a mut Stdout) -> Self {
        let mut towers = Vec::new();
        let resolution = Self::get_resolution();

        return Self {tower_list: towers, resolution: resolution, stdout: stdout};
    }

    pub fn place(&mut self, x: u16, heigth: u16) {
        let mut new_tower = Tower { x: x, heigth: heigth };
        new_tower.print(self.resolution, &mut self.stdout, Rgb(255, 255, 255));
        self.tower_list.push(new_tower);
        self.stdout.flush().unwrap();
        
    }

    pub fn swap(&mut self, x_l: u16, x_r: u16) {
        let x_l_u = x_l as usize;
        let x_r_u = x_r as usize;

        self.tower_list[x_l_u].clear(self.resolution, &mut self.stdout);
        self.tower_list[x_r_u].clear(self.resolution, &mut self.stdout);
        self.stdout.flush().unwrap();
        
        let tower1heigth = self.tower_list[x_l_u].get_heigth();
        let tower2heigth = self.tower_list[x_r_u].get_heigth();

        self.tower_list[x_l_u] = Tower { x: x_l, heigth: tower2heigth };
        self.tower_list[x_r_u] = Tower { x: x_r, heigth: tower1heigth };
        
        self.tower_list[x_l_u].print(self.resolution, &mut self.stdout, Rgb(255, 0, 0));
        self.tower_list[x_r_u].print(self.resolution, &mut self.stdout, Rgb(255, 0, 0));
        self.stdout.flush().unwrap();
        self.tower_list[x_l_u].print(self.resolution, &mut self.stdout, Rgb(255, 255, 255));
        self.tower_list[x_r_u].print(self.resolution, &mut self.stdout, Rgb(255, 255, 255));
    }

    pub fn verify(&mut self, x: u16) {
        self.tower_list[x as usize].print(self.resolution, &mut self.stdout, Rgb(0, 255, 0));
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        self.stdout.flush().unwrap();
        self.tower_list = Vec::new();
    }

    fn get_resolution() -> (u16, u16) {
        return termion::terminal_size().expect("terminalsize unknown");
    }
}

pub fn run(receiver: Receiver<[u16; 3]>) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    let mut towers = Towers::new(&mut stdout);
     
    loop {
        let message = receiver.recv().unwrap();

        match message[0] {
            0 => {towers.place(message[1], message[2])},
            1 => {towers.swap(message[1], message[2])},
            2 => {towers.verify(message[2])},
            3 => {towers.clear()},
            _ => panic!("unknown message between threads"),
        }
    }
}

