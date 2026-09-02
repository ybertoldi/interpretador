#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    Str(String), // TODO: utilizar referencia ao inves de copia
    Null,
}

use Object::*;
impl Object {
    pub fn equal(self, rhs: Self) -> Self {
        if self == rhs {
            Boolean(true)
        } else {
            Boolean(false)
        }
    }
    pub fn nequal(self, rhs: Self) -> Self {
        use Object::*;
        if self != rhs {
            Boolean(true)
        } else {
            Boolean(false)
        }
    }
    pub fn add(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Str(s1), Str(s2)) => Str(s1.to_string() + s2),
            (Number(n1), Number(n2)) => Number(n1 + n2),
            _ => panic!("can not add {:?} to {:?}", self, rhs),
        }
    }
    pub fn sub(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 - n2),
            _ => panic!("can not sub {:?} to {:?}", self, rhs),
        }
    }
    pub fn div(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 / n2),
            _ => panic!("can not div {:?} to {:?}", self, rhs),
        }
    }
    pub fn mult(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Str(s1), Number(n)) => Str(s1.repeat(*n as usize)),
            (Number(n1), Number(n2)) => Number(n1 * n2),
            _ => panic!("can not multiply {:?} to {:?}", self, rhs),
        }
    }
    pub fn less(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 < n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }
    pub fn less_eq(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 <= n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }
    pub fn greater(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 > n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }
    pub fn greater_eq(self, rhs: Self) -> Self {
        match (&self, &rhs) {
            (Number(n1), Number(n2)) => Boolean(n1 >= n2),
            _ => panic!("can not cmp {:?} to {:?}", self, rhs),
        }
    }

    pub fn bang(self) -> Self {
        match &self {
            Boolean(b) => Boolean(!b),
            _ => panic!("Can not apply bang to {:?}", self),
        }
    }
    pub fn negate(self) -> Self {
        match &self {
            Number(n) => Number(-n),
            _ => panic!("Can not apply unary negation to {:?}", self),
        }
    }

    pub fn and(&self, right: Object) -> Object {
        Object::Boolean(self.truth_value() && right.truth_value())
    }

    pub fn or(&self, right: Object) -> Object {
        Object::Boolean(self.truth_value() || right.truth_value())
    }

    pub fn truth_value(&self) -> bool {
        match self {
            Object::Boolean(b) => *b,
            Object::Number(n) => *n != 0.0,
            Object::Str(s) => !s.is_empty(),
            _ => false,
        }
    }
}
