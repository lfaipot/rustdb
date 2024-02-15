pub type VariableRef = String;
pub type VariableDesc = String;
pub type VariableGroup = String;
pub type VariableOperation = String;

/// trait common to all variable types
pub trait PrintInfo {
    fn print_info(&self);
}

pub enum VarSpecific {
    Toogle(VarToogle),
    Numeric(VarNumeric),
    Sensor4_20(VarSensor4_20),
}

pub union VarValue {
    pub int: i128,
    pub uint: u128,
    pub float: f64,
    pub boolean: bool,
}

impl VarSpecific {
    fn print_info(&self) {
        match self {
            VarSpecific::Toogle(toogle) => toogle.print_info(),
            VarSpecific::Numeric(numeric) => numeric.print_info(),
            VarSpecific::Sensor4_20(sensor) => sensor.print_info(),
        }
    }

    fn set_value(&mut self, varvalue: VarValue) {
        match self {
            VarSpecific::Toogle(toogle) => toogle.set_value(varvalue),
            VarSpecific::Numeric(numeric) => numeric.set_value(varvalue),
            VarSpecific::Sensor4_20(sensor) => sensor.set_value(varvalue),
        }
    }

}
//
// VARIABLE implementation, common to all variable types
//

pub struct Variable {
    reference: VariableRef,
    description: VariableDesc,
    group: VariableGroup,
    operation: VariableOperation,
    initialized: bool,
    specific: VarSpecific,
    used_by: Vec<String>,
}

impl Variable {
    pub fn new(reference: VariableRef, description: VariableDesc, group: VariableGroup, operation: VariableOperation, specific: VarSpecific) -> Self {
        println!("-----> GROUP:   {}", group);
        Self {
            reference,
            description,
            group,
            operation,
            initialized: false,
            specific,
            used_by: vec![],
        }
    }

    pub fn print_info(&self) {
        println!("");
        println!("Reference:   {}", self.reference);
        println!("Description: {}", self.description);
        println!("Group:       {}", self.group);
        println!("Operation:   {}", self.operation);
        println!("Initialized  {}", self.initialized);
        self.specific.print_info();
    }
    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn get_group(&self) -> String {
        return self.group.clone();
    }

    pub fn set_value(&mut self, varvalue: VarValue) {
        self.specific.set_value(varvalue);
        self.initialize();
    }
}


//
// TOOGLE (ON/OFF) implementation
//

pub struct VarToogle {
    value: bool,
}

impl VarToogle {
    pub fn new() -> VarSpecific {
        let toogle: VarToogle = VarToogle {
            value: false,
        };
        return VarSpecific::Toogle(toogle);
    }

    pub fn get(&self) -> bool {
        return self.value;
    }

    pub fn print_info(&self) {
        println!("Value:       {}", self.value);
    }

    pub fn set_value(&mut self, varvalue: VarValue) {
        let mut value = false;
        unsafe {
            value = varvalue.boolean;
        }
        let current_value: bool = self.value;
        let mut changed: bool = false;
        if value != current_value {
            changed = true;
        }
        self.value = value;
        println!("");
        println!("Set: ");
        if changed {
            println!("  Changed from {} to {}", current_value, self.value);
        } else {
            println!("  No change (still {})", self.value)
        }
    }
}

//
// INTEGER 
//

pub union NumericValue {
    pub int: i128,
    pub uint: u128,
    pub float: f64,
}
pub struct NumericFormat {
    pub integer: bool,
    pub signed: bool,
    pub size: u8,
}
pub struct VarNumeric {
    value: VarValue,
    format: NumericFormat,
    min: NumericValue,
    max: NumericValue,
}

impl VarNumeric {
    pub fn new(format: NumericFormat) -> VarSpecific {
        let numeric: VarNumeric = VarNumeric {
            value: VarValue {uint: 0},
            format,
            min: NumericValue {uint: 0},
            max: NumericValue {uint: 0},
        };
        return VarSpecific::Numeric(numeric);
    }

    pub fn print_info(&self) {
        println!("integer: {}", self.format.integer);
        println!("signed: {}", self.format.signed);
        println!("size: {}", self.format.size);
        unsafe {
            if self.format.integer {
                if self.format.signed {
                    println!("int:   {}", self.value.int);
                } else {
                    println!("uint:  {}", self.value.uint);
                }
            } else {
                println!("float: {}", self.value.float);
            }
        }
    }

    pub fn set_value(&mut self, value: VarValue) {
        if self.format.integer {
            if self.format.signed {
                unsafe {
                    let current_value = self.value.int;
                    let new_value = value.int;
                    self.value.int = new_value;
                }
            } else {
                unsafe {
                    let current_value = self.value.uint;
                    let new_value = value.uint;
                    self.value.uint = new_value;
                }
            }
        } else {
            unsafe {
                let current_value = self.value.float;
                let new_value = value.float;
                self.value.float = new_value;
            }

        }
    }
}

//
//  4-20mA SENSOR{}
// 

pub type SensorValue = u128;
pub type ActualValue = f64;
pub type RangeValue = u64;

pub struct VarSensor4_20 {
    value: ActualValue,
    val4ma: ActualValue,
    val20ma: ActualValue,
    range: RangeValue,
    sensor_value: SensorValue,
}

impl VarSensor4_20 {
    pub fn new(val4ma: ActualValue, val20ma: ActualValue, range: RangeValue) -> VarSpecific {
        let sensor4_20: VarSensor4_20 = VarSensor4_20 {
            value: 0.0,
            val4ma,
            val20ma,
            range,
            sensor_value: 0 as SensorValue,
        };
        return VarSpecific::Sensor4_20(sensor4_20);
    }

    pub fn print_info(&self) {
        println!("Sensor value:   {}", self.sensor_value);
        println!("4 mA value:     {}", self.val4ma);
        println!("20 mA:          {}", self.val20ma);
        println!("Actual value    {}", self.value);
    }

    pub fn set_value(&mut self, varvalue: VarValue) {
        // physical value is from 0 to 255
        let value;
        unsafe {
            value = varvalue.uint;
        }
        self.sensor_value = value;
        self.value = (self.val20ma - self.val4ma) / (self.range as ActualValue) * (value as ActualValue) + self.val4ma;
    }
}

