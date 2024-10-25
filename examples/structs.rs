struct Employee<'a> {
    emp_id: u32,
    emp_name: &'a str,
}

/// Difference between `<'a>` and `<'_>` ?
impl Employee<'_> {
    pub fn new(emp_id: u32, emp_name: &str) -> Employee {
        Employee { emp_id: emp_id, emp_name: emp_name }
    }

    pub fn to_string(&self) -> String {
        format!("员工编号：{:?}，员工姓名：{:?}", self.emp_id, self.emp_name)
    }
}

///
#[cfg(test)]
pub mod structs_test_cases {
    use crate::Employee;

    #[test]
    pub fn test_structs1() {
        let emp = Employee::new(121001, "xien.sxe");
        let to_string = emp.to_string();
        println!("{:?}", to_string);
    }
}

/// No `main` function found in crate `structs` [EO601]
fn main() {}