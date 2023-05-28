use std::sync::mpsc::Sender;
use std::{thread, time, vec::Vec};
use rand::{thread_rng, seq::SliceRandom};

static SWAP_TIME: u64 = 20; // time between swaps in ms
static VERIFY_TIME: u64 = 30; // time between verify in ms

struct Sorting {
    list: Vec<usize>,
    size: usize,
    transmitter: Sender<[u16; 3]>,
}

impl Sorting {
    pub fn new(transmitter: Sender<[u16; 3]>) -> Self {
        return Self { 
            list: Vec::new(), 
            size: (termion::terminal_size().expect("terminalsize unknown").0) as usize, 
            transmitter: transmitter
        };
    }

    pub fn swap_items(&mut self, x_l: usize, x_r: usize) {
        let number_l = self.list[x_l];
        
        self.list[x_l] = self.list[x_r];
        self.list[x_r] = number_l;

        self.transmitter.send([1, x_l as u16, x_r as u16]).expect("couldn't send message to output thread");
        thread::sleep(time::Duration::from_millis(SWAP_TIME));
    }

    pub fn verify(&self) -> bool {
        self.transmitter.send([2, 0, 0]).expect("couldn't send message to output thread");
        for i in 1..self.list.len()-1 {
            if self.list[i-1] < self.list[i] && self.list[i] < self.list[i+1] {
                self.transmitter.send([2, 0, i as u16]).unwrap();
                thread::sleep(time::Duration::from_millis(VERIFY_TIME));
            } else {
                return false;
            }
        }
        self.transmitter.send([2, 0, (self.size as u16)-1]).expect("couldn't send message to output thread");
        return true;
    }

    pub fn randomize(&mut self) {
        self.list = (0..self.size).collect();
        self.transmitter.send([3, 0, 0]).expect("couldn't send message to output thread");

        self.list.shuffle(&mut thread_rng());
        for i in 0..self.size {
            self.transmitter.send([0, i as u16, self.list[i] as u16]).expect("couldn't send message to output thread");
        }
    }
}

// Space for sorting algorithms:

pub struct BubbleSort {
    sorting: Sorting,
}

impl BubbleSort {
    pub fn new(transmitter: Sender<[u16; 3]>) -> Self {
        let mut sorting = Sorting::new(transmitter);
        sorting.randomize();
        return Self { sorting: sorting };
    }


    pub fn sort(&mut self) {
        let len = self.sorting.list.len();
        let mut swapped;

        loop {
            swapped = false;

            for i in 0..len-1 {
                if self.sorting.list[i] > self.sorting.list[i+1] {
                    self.sorting.swap_items(i, i+1);
                    swapped = true;
                }
            }

            if !swapped {
                self.sorting.verify();
                break;
            }
        }
    }
}

