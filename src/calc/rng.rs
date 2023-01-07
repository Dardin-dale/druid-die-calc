use rand::rng;

struct dieRoller {
    total: i32,
    cmd: String,
    die_rolls: Vec<Vec<i32>>,
    consts: Vec<i32>,
}

struct dieCmd {
    num_die: i32,
    num_sides: i32,
}



impl dieRoller {
    fn parseCmd(&mut self, cmd: &str) {
        //loop through command string separate by operands
        

            //grab if die command
        
            //grab if constant

    }

    pub fn roll(&mut self, cmd: String) -> dieRoller {
        self.cmd = cmd;
    }

    pub fn reroll(&mut self) -> dieRoller {

    }

    pub fn display(&mut self) {

    }
}
