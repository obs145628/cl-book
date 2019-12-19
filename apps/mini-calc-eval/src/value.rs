

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Value
{
    VInt(i64),
    VFloat(f64)
}

impl Value {

    pub fn is_int(&self) -> bool {
	match self {
	    Value::VInt(_) => true,
	    _ => false,
	}
    }

    pub fn is_float(&self) -> bool {
	match self {
	    Value::VFloat(_) => true,
	    _ => false,
	}
    }

    pub fn get_float(&self) -> f64 {
	match self {
	    Value::VInt(x) => *x as f64,
	    Value::VFloat(x) => *x,
	}
    }

    pub fn get_int(&self) -> i64 {
	match self {
	    Value::VInt(x) => *x,
	    Value::VFloat(x) => *x as i64,
	}
    }
    
    pub fn add(x: &Value, y: &Value) -> Value {
	if x.is_int() && y.is_int() {
	    Value::VInt(x.get_int() + y.get_int()) 
	} else {
	    Value::VFloat(x.get_float() + y.get_float())
	}
    }

    pub fn sub(x: &Value, y: &Value) -> Value {
	if x.is_int() && y.is_int() {
	    Value::VInt(x.get_int() + y.get_int()) 
	} else {
	    Value::VFloat(x.get_float() + y.get_float())
	}
    }

    pub fn mul(x: &Value, y: &Value) -> Value {
	if x.is_int() && y.is_int() {
	    Value::VInt(x.get_int() + y.get_int()) 
	} else {
	    Value::VFloat(x.get_float() + y.get_float())
	}
    }

    pub fn div(x: &Value, y: &Value) -> Value {
	if x.is_int() && y.is_int() {
	    Value::VInt(x.get_int() + y.get_int()) 
	} else {
	    Value::VFloat(x.get_float() + y.get_float())
	}
    }
    

}
