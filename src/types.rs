#[derive(Debug, Clone)]
pub enum State {
    MULT, ADD, PRINT
}

#[derive(Debug, Clone)]
pub struct Product {
    size: usize,
    v1: Vec<i32>,
    v2: Vec<i32>,
    v3: Vec<i32>,
    result: i32,
    pending_mult: Vec<i32>,
    pub state: State
}

impl Product {
    pub fn new() -> Product {

        Product {
            size: 4,
            v1: vec![],
            v2: vec![],
            v3: vec![],
            result: 0,
            pending_mult: vec![],
            state: State::MULT
        }
    }

    pub fn init(&mut self) {
        // Initialisation de pending et des tableaux v.
        for _ in 0..self.size {
            self.pending_mult.push(1);
            self.v1.push(0);
            self.v2.push(0);
            self.v3.push(0);
        }
    }

    pub fn set_pending(&mut self, idx: usize, value: i32) {
        self.pending_mult[idx] = value
    }

    pub fn set_v1(&mut self, index: usize, value: i32) {
        self.v1[index] = value
    }

    pub fn set_v2(&mut self, index: usize, value: i32) {
        self.v2[index] = value
    }

    pub fn set_v3(&mut self, index: usize, value: i32) {
        self.v3[index] = value
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_v1(&self, index: usize) -> i32 {
        self.v1[index]
    }

    pub fn get_v2(&self, index: usize) -> i32 {
        self.v2[index]
    }

    pub fn get_nb_pending(&self) -> i32 {
        let mut sum = 0;
        for idx in 0..self.size {
            sum += self.pending_mult[idx];
        }
        sum
    }

    pub fn get_result(&self) -> i32 {
        self.result
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn find_final_result(&mut self) {
        let mut sum = 0;
        for idx in 0..self.size {
            sum += self.v3[idx];
        }
        self.result = sum;
    }
}