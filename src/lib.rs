use db::{BigNumber, Datatype};
use pyo3::prelude::*;
mod db;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    println!("Rust: {} + {}", a, b);
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn z33(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PYDB>()?;
    Ok(())
}

#[pyclass]
struct PYDB {
    db: db::DB,
}

#[pymethods]
impl PYDB {
    #[new]
    fn new(name: &str, raw_fp: &str, ov_fp: &str) -> PyResult<Self> {
        Ok(PYDB {
            db: db::DB::new(name, raw_fp, ov_fp),
        })
    }

    fn init(&mut self) {
        self.db.add_workflow("workflow1").unwrap();

        let workflow = self.db.get_mut_workflow("workflow1").unwrap();
    
        workflow.add_resource("resource1").unwrap();
        workflow.add_container("container1").unwrap();
        workflow.add_store("store1").unwrap();
        workflow.add_custom("custom1").unwrap();
    
        let resource1 = workflow.get_mut_resource("resource1").unwrap();
        resource1.add_user_time("user1", db::BigNumber::Int(70), db::BigNumber::Int(80)).unwrap();
        resource1.add_enter_time("user1", db::BigNumber::Int(89)).unwrap();
        resource1.add_leave_time("user1", db::BigNumber::Int(90)).unwrap();
    
    }

    fn add_workflow(&mut self, name: &str) {
        self.db.add_workflow(name).unwrap();
    }

    fn add_resource(&mut self, workflow: &str, name: &str) {
        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        workflow.add_resource(name).unwrap();
    }

    fn add_container(&mut self, workflow: &str, name: &str) {
        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        workflow.add_container(name).unwrap();
    }

    fn add_store(&mut self, workflow: &str, name: &str) {
        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        workflow.add_store(name).unwrap();
    }

    fn add_custom(&mut self, workflow: &str, name: &str) {
        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        workflow.add_custom(name).unwrap();
    }

    fn res_add_user_time(&mut self, py: Python, workflow: &str, resource: &str, user: &str, jtime: PyObject, qlen: PyObject) {

        let jtime = process_bignum(py, jtime);
        let qlen = process_bignum(py, qlen);

        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        let resource = workflow.get_mut_resource(resource).unwrap();
        resource.add_user_time(user, jtime, qlen).unwrap();
    }

    fn res_add_enter_time(&mut self, py: Python, workflow: &str, resource: &str, user: &str, jtime: PyObject) {
        let jtime = process_bignum(py, jtime);

        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        let resource = workflow.get_mut_resource(resource).unwrap();
        resource.add_enter_time(user, jtime).unwrap();
    }

    fn res_add_leave_time(&mut self, py: Python, workflow: &str, resource: &str, user: &str, jtime: PyObject) {
        let jtime = process_bignum(py, jtime);

        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        let resource = workflow.get_mut_resource(resource).unwrap();
        resource.add_leave_time(user, jtime).unwrap();
    }

    fn cont_put_time(&mut self, py: Python, workflow: &str, container: &str, ptime: PyObject, pamount: PyObject, tamount: PyObject, user: &str) {
        let ptime = process_bignum(py, ptime);
        let pamount = process_bignum(py, pamount);
        let tamount = process_bignum(py, tamount);

        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        let container = workflow.get_mut_container(container).unwrap();
        container.add_put_time(ptime, pamount, tamount, user).unwrap();
    }

    fn cont_get_time(&mut self, py: Python, workflow: &str, container: &str, gtime: PyObject, gamount: PyObject, tot: PyObject,user: &str) {
        let gtime = process_bignum(py, gtime);
        let gamount = process_bignum(py, gamount);
        let tot = process_bignum(py, tot);

        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        let container = workflow.get_mut_container(container).unwrap();
        container.add_get_time(gtime, gamount, tot, user).unwrap();
    }

    fn store_put_time(&mut self, py: Python, workflow: &str, store: &str, ptime: PyObject, pamount: PyObject, tamount: PyObject, user: &str) {
        let ptime = process_bignum(py, ptime);
        let pamount = process_bignum(py, pamount);
        let tamount = process_bignum(py, tamount);

        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        let store = workflow.get_mut_store(store).unwrap();
        store.add_put_time(ptime, pamount, tamount, user).unwrap();
    }

    fn store_get_time(&mut self, py: Python, workflow: &str, store: &str, gtime: PyObject, gamount: PyObject, tot: PyObject, user: &str) {
        let gtime = process_bignum(py, gtime);
        let gamount = process_bignum(py, gamount);
        let tot = process_bignum(py, tot);

        let workflow = self.db.get_mut_workflow(workflow).unwrap();
        let store = workflow.get_mut_store(store).unwrap();
        store.add_get_time(gtime, gamount, tot, user).unwrap();
    }
    
    fn printdb(&self) {
        let workflow = self.db.get_workflow("workflow1").unwrap();

        println!("{:?}", workflow);
    }
}



fn process_bignum(py: Python, value: PyObject) -> BigNumber{
    match value.extract::<i64>(py) {
        Ok(f) => BigNumber::Int(f),
        Err(_) => {
            match value.extract::<f64>(py) {
                Ok(i) => BigNumber::Float(i),
                Err(_) => {
                    panic!("Unsupported type");
                }
            }
        }
    }
    
}