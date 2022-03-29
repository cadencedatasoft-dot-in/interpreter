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
    "RETURN_VALUE",

    "LOAD_VAL 1", 
    "WRITE_VAR y",    
    "LOAD_VAL 1",
    "WRITE_VAR z", 

    "LOOPW_START myid z", //LOOP_START <ID> <BOOL>, this is a while loop 
    "READ_VAR y",
    "LOAD_VAL 1",
    "ADD",
    "WRITE_VAR y",
    "LOOPW_END myid", //LOOP_END <ID> //Should match with START
    "EXIT"];

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

pub fn execute_prog<'a>(prog_mem: Vec<(&str, Operands)>, inst_set: & HashMap<String, i8>) -> bool{
    let mut retval = true;

    let mut data_mem: VecDeque<i64> = VecDeque::with_capacity(STACK_MAX);
    let mut vars = HashMap::new();

    //for inst in prog_mem {
    let mut jmp_offset: i32 = 0;
    let mut i: i32 = -1;
    //for mut i in 0..prog_mem.len() {    
    loop {
        i += 1;
        i = (i + jmp_offset);
        jmp_offset = 0;
        let inst = prog_mem.get(i as usize).unwrap();
        let instruction = inst.0;
        match instruction {
            "LOAD_VAL" => {
                let opd1 = (&inst).1.opdval1.unwrap();
                if (data_mem.len() + 1) <  STACK_MAX {
                    data_mem.push_back(opd1);
                }else{
                    println!("Stack overflow");
                    retval = false;
                    break;
                }
            },
            "WRITE_VAR" => {
                let opd1 = inst.1.opdname1.as_ref().unwrap();
                let val_idx = match vars.get(opd1){
                    Some(v) => *v,
                    None => {
                        //New variable, not already declared
                        let curr_top: i32 = (data_mem.len() - 1) as i32;
                        vars.insert(opd1, curr_top);  //Value for variable resides here. Are we allowing unlimited variables? 
                        continue;
                    }
                };

                if val_idx <= data_mem.len() as i32 {
                    let lastval = data_mem.pop_back().unwrap();
                    if let Some(elem) = data_mem.get_mut((val_idx) as usize) {
                        *elem = lastval;
                    }
                }               
            },
            "READ_VAR" => {
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
            "LOOPW_START" => {
                //Get operand count for validation
                let oprrnds_count = *inst_set.get(inst.0).unwrap();

                //Retrive condition expression
                let condition_var = inst.1.opdname2.clone().unwrap();

                let cond_val = match should_loop(&condition_var, &vars, &data_mem){
                    Some(v) => v,
                    None => {
                        println!("Unknown error");
                        retval = false;
                        break;
                    }
                };

                //Evaluate condition expression
                if cond_val == true {
                    
                    continue; 
                }else{
                //If NOT condition satisfied, start executing instruction after the LOOPW_END_
                    let end_marker = inst.1.opdname1.clone().unwrap();

                    let offset = match loopend_offset(&end_marker, &prog_mem, i){
                        Some(v) => v,
                        None => {
                            println!("Unknown error");
                            retval = false;
                            break;                            
                        }
                    };
                    //Add one because we are going to instruction after LOOPW_END
                    jmp_offset = offset as i32 + 1 -1; //Substract one because PC in going to increment before it starts executing next instruction
                }
            },
            "LOOPW_END" => {
                let oprrnds_count = *inst_set.get(inst.0).unwrap();

                let start_marker = inst.1.opdname1.clone().unwrap();
                let os = match loopstart_offset(&start_marker, &prog_mem, i){
                    Some(v) => v,
                    None => {
                        println!("Unknown error");
                        retval = false;
                        break;                            
                    }
                };
                jmp_offset = -(os as i32 + 1); //Add one because PC in going to increment before it starts executing next instruction
            },
            "EXIT" => {
                println!("Exiting program");
            },                        
            _ => {
                println!("Unknown instruction cannot process");
            }
        }
    }

    return retval;
}

fn should_loop(loop_var: &String, vars: &HashMap<&String, i32>, data_mem: &VecDeque<i64>) -> Option<bool> {
    let mut retval = false;
    let val_idx = *vars.get(&loop_var).unwrap();

    if val_idx <= data_mem.len() as i32 {
        let val = match data_mem.get(val_idx as usize) {
            Some(v) => *v,
            None => {
                //data_mem.push_back(loop_var.clone())
                return None;
            }
        };

        if val > 0 {
            retval = true;
        }
    }else{
        println!("Variable is out of scope");
    }

    return Some(retval);
}

fn loopend_offset(end_marker: &String, prog_mem: &Vec<(&str, Operands)>, pc: i32) -> Option<i32> {
    let mut loop_end = String::from("LOOPW_END ");

    let offset = match prog_mem.iter().position(|x | {
        let p1 = x.0;
        let p2 = match &x.1.opdname1{
            Some(v) => v,
            None => return false
        };

        if p1.contains(&loop_end) {
            if p2.contains(end_marker) {
                return true
            }
        }

        return false;
    }){
        Some(v) => v as i32,
        None => {
            return None;
        }
    };

    return Some(offset as i32);
}

fn loopstart_offset(start_marker: &String, prog_mem: &Vec<(&str, Operands)>, pc: i32) -> Option<i32> {
    let mut loop_end = String::from("LOOPW_START");

    let offset = match prog_mem.iter().position(|x | {
        let p1 = x.0;
        let p2 = match &x.1.opdname1{
            Some(v) => v,
            None => return false
        };

        if p1.contains(&loop_end) {
            if p2.contains(start_marker) {
                return true
            }
        }

        return false;
    }){
        Some(v) => pc-v as i32,
        None => {
            return None;
        }
    };

    return Some(offset);
}


pub fn validate_and_load_prog<'a>(prog: &[&'a str;21], iset: & HashMap<String, i8>, programmemory: &mut Vec<(&'a str, Operands)>) -> bool{
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
                "READ_VAR" |
                "LOOPW_END" => {
                    if toks.len() != 2 {
                        println!("Invalid argument count");
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
                            "LOOPW_END" => {
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
                "DIVIDE" |
                "EXIT" => {
                    if toks.len() != 1 {
                        println!("Invalid argument count");
                        retval = false;
                        break;
                    }else{
                        programmemory.push((mnem, Operands{ opdval1: None, opdval2: None, opdname1: None, opdname2: None}) );
                    }                    
                },
                "LOOPW_START" => {
                    if toks.len() != 3 {
                        println!("Invalid argument count");
                        retval = false;
                        break;
                    }else{
                        let id = toks[1].to_string();
                        let mut end = String::from("LOOPW_END ");
                        end.push_str(&id);

                        if prog.contains(&end.as_str()) {
                            programmemory.push((mnem, Operands{ opdval1: None, opdval2: None, opdname1: Some(toks[1].to_string()), opdname2: Some(toks[2].to_string())}) );
                        }else{
                            println!("Syntax error, LOOPW end not found");
                        }
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
    set.insert(String::from("LOOPW_START"), 2); 
    set.insert(String::from("LOOPW_END"), 1);
    set.insert(String::from("EXIT"), 0);
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