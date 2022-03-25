use std::collections::VecDeque;
use std::collections::HashMap;

pub struct Operands {
    opdval1: Option<i64>,
    opdval2: Option<i64>,
    opdname1: Option<String>,
    opdname2: Option<String>,
}

const STACK_MAX: usize = 100;
fn main() {
    let submitted_prog = [
    "LOAD_VAL 5", 
    "WRITE_VAR x",
    "LOAD_VAL 2",
    "WRITE_VAR y",
    "READ_VAR x",
    "LOAD_VAL 1",
    "ADD",
    "READ_VAR y",
    "MULTIPLY",
    "RETURN_VALUE"];

    //Instruction set holds instruction & operand count
    let mut inst_set = HashMap::new();
    init_inst_set(&mut inst_set);

    //Create memory to load the program into
    let mut prog_mem = Vec::new();

    //Validate byte code
    if !validate_and_load_prog(&submitted_prog, &inst_set, &mut prog_mem){ 
        println!("The ByteCode is ot supported by this ALU");
        return;
    }

    //Execute byte code
    if execute_prog(prog_mem, &inst_set) {
        println!("Successfully executed program");
    }
}

pub fn execute_prog(prog_mem: Vec<(&str, Operands)>, inst_set: & HashMap<String, i8>) -> bool{
    let mut retval = true;

    let mut data_mem: VecDeque<i64> = VecDeque::with_capacity(STACK_MAX);
    let mut vars = HashMap::new();

    for inst in prog_mem {
        let instruction = inst.0;
        match instruction {
            "LOAD_VAL" => {
                let opd1 = inst.1.opdval1.unwrap();
                if (data_mem.len() + 1) <  STACK_MAX {
                    data_mem.push_back(opd1);
                }else{
                    println!("Stack overflow");
                    retval = false;
                    break;
                }
            },
            "WRITE_VAR" => {
                let opd1 = inst.1.opdname1.unwrap();
                let curr_top: i32 = (data_mem.len() - 1) as i32;
                //Are we allowing ubnlimited variables?
                vars.insert(opd1, curr_top); //this is where value to opd1 resides
            },
            "READ_VAR"  => {
                let opd1 = inst.1.opdname1.as_ref().unwrap();
                let val_idx = vars.get(opd1).unwrap();
                if *val_idx <= data_mem.len() as i32 {
                    let val = *data_mem.get((*val_idx) as usize).unwrap();
                    if (data_mem.len() + 1) <  STACK_MAX {
                        data_mem.push_back(val);
                    }else{
                        println!("Stack overflow");
                        retval = false;
                        break;
                    }                    
                }else{
                    println!("Variable is out of scope");
                    retval = false;
                    break;
                }
            },
            "RETURN_VALUE" => {
                if data_mem.len() > 0 {
                    let retval = data_mem.pop_back().unwrap();
                    println!("Result: {}", retval);
                }else{
                    println!("Stack underflow");
                    retval = false;
                    break;
                }                
            },
            "ADD" => {
                let oprrnds_count = *inst_set.get(inst.0).unwrap();
                if data_mem.len() >= oprrnds_count as usize {
                    let oprand1 = data_mem.pop_back().unwrap();
                    let oprand2 = data_mem.pop_back().unwrap();
                    data_mem.push_back( oprand1 + oprand2);
                }else{
                    println!("Stack underflow");
                    retval = false;
                    break;
                }                    
            },
            "SUBTRACT" => {
                let oprrnds_count = *inst_set.get(inst.0).unwrap();
                if data_mem.len() >= oprrnds_count as usize {
                    let oprand1 = data_mem.pop_back().unwrap();
                    let oprand2 = data_mem.pop_back().unwrap();
                    data_mem.push_back( oprand1 - oprand2);
                }else{
                    println!("Stack underflow");
                    retval = false;
                    break;
                }                    
            },
            "MULTIPLY" => {
                let oprrnds_count = *inst_set.get(inst.0).unwrap();
                if data_mem.len() >= oprrnds_count as usize {
                    let oprand1 = data_mem.pop_back().unwrap();
                    let oprand2 = data_mem.pop_back().unwrap();
                    data_mem.push_back( oprand1 * oprand2);
                }else{
                    println!("Stack underflow");
                    retval = false;
                    break;
                }                    
            },
            "DIVIDE" => {
                let oprrnds_count = *inst_set.get(inst.0).unwrap();
                if data_mem.len() >= oprrnds_count as usize {
                    let oprand1 = data_mem.pop_back().unwrap();
                    let oprand2 = data_mem.pop_back().unwrap();
                    data_mem.push_back( oprand1 / oprand2);
                }else{
                    println!("Stack underflow");
                    retval = false;
                    break;
                }                    
            },
            _ => {
                println!("Unknown instruction cannot process");
            }
        }
    }

    return retval;
}


pub fn validate_and_load_prog<'a>(prog: &[&'a str;10], iset: & HashMap<String, i8>, programmemory: &mut Vec<(&'a str, Operands)>) -> bool{
    let mut retval = true;
    
    for inst in prog{
        let toks: Vec<&str> = inst.split(' ').collect();
        let mnem = toks[0];

        if iset.get(mnem).is_none() {
            retval = false;
            programmemory.clear();
            break;
        }else{
            match mnem {
                "LOAD_VAL" | 
                "WRITE_VAR" |
                "READ_VAR" => {
                    if toks.len() != 2 {
                        retval = false;
                        break;
                    }else{

                        match mnem {
                            "LOAD_VAL" => {
                                programmemory.push((mnem, Operands{ opdval1: Some(toks[1].parse::<i64>().unwrap()), opdval2: None, opdname1: None, opdname2: None}) );
                            },
                            "WRITE_VAR" => {
                                programmemory.push((mnem, Operands{ opdval1: None, opdval2: None, opdname1: Some(toks[1].to_string()), opdname2: None}) );
                            },
                            "READ_VAR" => {
                                programmemory.push((mnem, Operands{ opdval1: None, opdval2: None, opdname1: Some(toks[1].to_string()), opdname2: None}) );
                            },
                            _ => {
                                println!("Invalid arguments")
                            }
                        }                            
                    }
                },
                "RETURN_VALUE" |
                "ADD" |
                "SUBTRACT" |
                "MULTIPLY" |
                "DIVIDE" => {
                    if toks.len() != 1 {
                        retval = false;
                        break;
                    }else{
                        programmemory.push((mnem, Operands{ opdval1: None, opdval2: None, opdname1: None, opdname2: None}) );
                    }                    
                },
                _ => {
                    println!("Invalid syntax")
                }
            }
        }
    }

    return retval;
}

//Fixed instruction set
pub fn init_inst_set(set: &mut HashMap<String, i8>){
    set.insert(String::from("LOAD_VAL"), 1);
    set.insert(String::from("READ_VAR"), 1);
    set.insert(String::from("WRITE_VAR"), 1);
    set.insert(String::from("ADD"), 2);
    set.insert(String::from("SUBTRACT"), 2);
    set.insert(String::from("DIVIDE"), 2);
    set.insert(String::from("MULTIPLY"), 2);    
    set.insert(String::from("RETURN_VALUE"), 1);
}

//For now unused code 
pub struct  Instruct {
    kind: Instructions,
}
impl  Instruct{
    pub fn new(kind: Instructions) -> Self {
        Self { kind }
    }
}

pub enum Instructions {
    Noop,
    LoadVal, //PUSH
    ReadVar, //PUSH
    WriteVar, //POP
    Add, //MEMONICS
    Sub, //MEMONICS
    Mul, //MEMONICS
    Div, //MEMONICS
    Ret, //MEMONICS
}

pub trait Compute {
    fn compute(&self) -> i64;
    fn display(&self) -> String;
}

struct Noop{
}

struct LoadVal{
    literal: i64,
}

impl LoadVal{
    pub fn new(value: i64) -> Self {
        Self { literal: value }
    }
    pub fn literal(&self) -> i64 {
        self.literal
    }
    pub fn PUSH(gstack: &mut VecDeque<i64>, val: i64) {
        gstack.push_back(val)
    }
}

struct ReadVar{
    varname: String,
    value: i64
}
impl ReadVar{
    pub fn new(varname: String, value: i64) -> Self {
        Self { varname, value}
    }
    pub fn PUSH(gstack: &mut VecDeque<i64>, val: i64) {
        gstack.push_back(val);
    }    

    pub fn display(&self){
        println!("{} = {:?}", self.varname, self.value);
    }
}

struct WriteVar{
    varname: String,
    value: i64
}
impl WriteVar{
    pub fn new(varname: String, value: i64) -> Self {
        Self { varname, value }
    }
    pub fn POP(gstack: &mut VecDeque<i64>) -> Option<i64> {
        gstack.pop_back()
    }
    pub fn display(&self){
        println!("{} = {:?}", self.varname, self.value);
    }
}
struct Add{
    opcount: u8,
    result: i64
}
impl Add{
    pub fn new() -> Self {
        Self { opcount: 2, result: 0 }
    }

    pub fn COMPUTE(&mut self, gstack: &mut VecDeque<i64>) -> Result<i64, String>{

        let v1: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        let v2: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        self.result = v1 + v2;
        return Ok(self.result)
    }

    pub fn display(&self){
        //println!("ADD RESULT = {:?}", self.varname, self.value);
    }
}
struct Sub{
    opcount: u8,
    result: i64
}
impl Sub{
    pub fn new() -> Self {
        Self { opcount: 2, result: 0 }
    }

    pub fn COMPUTE(&mut self, gstack: &mut VecDeque<i64>) -> Result<i64, String>{

        let v1: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        let v2: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        self.result = v1 - v2;
        return Ok(self.result)
    }

    pub fn display(&self){
        //println!("ADD RESULT = {:?}", self.varname, self.value);
    }
}
struct Mul{
    opcount: u8,
    result: i64
}
impl Mul{
    pub fn new() -> Self {
        Self { opcount: 2, result: 0 }
    }

    pub fn COMPUTE(&mut self, gstack: &mut VecDeque<i64>) -> Result<i64, String>{

        let v1: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        let v2: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        self.result = v1 * v2;
        return Ok(self.result)
    }

    pub fn display(&self){
        //println!("ADD RESULT = {:?}", self.varname, self.value);
    }
}
struct Div{
    opcount: u8,
    result: i64
}
impl Div{
    pub fn new() -> Self {
        Self { opcount: 2, result: 0 }
    }

    pub fn COMPUTE(&mut self, gstack: &mut VecDeque<i64>) -> Result<i64, String>{

        let v1: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        let v2: i64 = match gstack.pop_back() {
            None => return Err(String::from("Error: Invalid logic, aborting")),
            Some(v) => v,
        };

        self.result = v1 / v2;
        return Ok(self.result)
    }

    pub fn display(&self){
        //println!("ADD RESULT = {:?}", self.varname, self.value);
    }
}