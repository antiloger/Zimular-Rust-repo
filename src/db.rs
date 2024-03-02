use std::collections::HashMap;

type DBResult<T> = std::result::Result<T, DBERROR>;

#[derive(Debug)]
pub enum DBERROR {
    DataAlreadyExist,
    DataNotExist,
}

#[derive(Debug)]
pub enum Datatype {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub enum BigNumber {
    Int(i64),
    Float(f64),
}

// #[derive(Debug)]
// enum ComponentType {
//     Resource(Resource),
//     Container(Container),
//     Store(Store),
//     Custom(CustomComponent),
// }

#[derive(Debug)]
pub struct DB {
    name: String,
    data: Box<HashMap<String, Workflow>>,
    size_in_bytes: i64,
    raw_file_path: String,
    overview_file_path: String,
    default_data: DefaultData,
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    resource: HashMap<String, Resource>,
    container: HashMap<String, Container>,
    store: HashMap<String, Store>,
    custom: HashMap<String, CustomComponent>,
    default_data: DefaultData,
    workflow_info: WorkflowInfo,
}

#[derive(Debug)]
pub struct Resource {
    user_time: Vec<(String, BigNumber, BigNumber)>, // (user, queue_join_time, queue_length)
    enter_time: Vec<(String, BigNumber)>,           // (user, enter_time)
    leave_time: Vec<(String, BigNumber)>,           // (user, leave_time)
    default_data: DefaultData,
    res_info: ResInfo,
}

#[derive(Debug)]
pub struct Container {
    put_time: Vec<(BigNumber, BigNumber, BigNumber, String)>, //(Put_time, put_amount, total_amount(after), user)
    get_time: Vec<(BigNumber, BigNumber, BigNumber, String)>, //(Get_time, get_amount, total_amount(after), user)
    default_data: DefaultData,
    cont_info: ContInfo,
}

#[derive(Debug)]
pub struct Store {
    put_time: Vec<(BigNumber, BigNumber, BigNumber, String)>, //(Put_time, put_amount, total_amount(after), user)
    get_time: Vec<(BigNumber, BigNumber, BigNumber, String)>, //(Get_time, get_amount, total_amount(after), user)
    default_data: DefaultData,
    store_info: StoreInfo,
}

#[derive(Debug)]
pub struct CustomComponent {
    des: String,
    default_data: DefaultData,
}

#[derive(Debug)]
pub struct ResInfo {
    avg_time: BigNumber,
    no_of_users: i64,
    leave: BigNumber,
    rawstore: bool,
    analysis: bool,
}

#[derive(Debug)]
pub struct ContInfo {
    top_get_amount: BigNumber,
    top_put_amount: BigNumber,
    clear_times: i64,
    rawstore: bool,
    analysis: bool,
}

#[derive(Debug)]
pub struct StoreInfo {
    top_get_amount: BigNumber,
    top_put_amount: BigNumber,
    clear_times: i64,
    rawstore: bool,
    analysis: bool,
}

#[derive(Debug)]
pub struct WorkflowInfo {
    resinfo: BigNumber,
}

#[derive(Debug)]
pub struct DefaultData {
    datatype: HashMap<String, Datatype>,
}



impl DB {
    pub fn new(name: &str, raw_file: &str, ov_file: &str) -> DB {
        DB {
            name: name.to_string(),
            data: Box::new(HashMap::new()),
            size_in_bytes: 0,
            raw_file_path: raw_file.to_string(),
            overview_file_path: ov_file.to_string(),
            default_data: DefaultData {
                datatype: HashMap::new(),
            },
        }
    }

    pub fn add_workflow(&mut self, name: &str) -> DBResult<()> {
        if self.data.contains_key(name) {
            return Err(DBERROR::DataAlreadyExist);
        }
        self.data.insert(name.to_string(), Workflow::new(name));
        Ok(())
    }

    pub fn get_workflow(&self, name: &str) -> DBResult<&Workflow> {
        match self.data.get(name) {
            Some(w) => Ok(w),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_mut_workflow(&mut self, name: &str) -> DBResult<&mut Workflow> {
        match self.data.get_mut(name) {
            Some(w) => Ok(w),
            None => Err(DBERROR::DataNotExist),
        }
    }
}

impl Workflow {
    pub fn new(name: &str) -> Workflow {
        Workflow {
            name: name.to_string(),
            resource: HashMap::new(),
            container: HashMap::new(),
            store: HashMap::new(),
            custom: HashMap::new(),
            default_data: DefaultData {
                datatype: HashMap::new(),
            },
            workflow_info: WorkflowInfo {
                resinfo: BigNumber::Int(0),
            },
        }
    }

    pub fn add_resource(&mut self, name: &str) -> DBResult<()> {
        if self.resource.contains_key(name) {
            return Err(DBERROR::DataAlreadyExist);
        }
        self.resource.insert(name.to_string(), Resource::new());
        Ok(())
    }

    pub fn add_container(&mut self, name: &str) -> DBResult<()> {
        if self.container.contains_key(name) {
            return Err(DBERROR::DataAlreadyExist);
        }
        self.container.insert(name.to_string(), Container::new());
        Ok(())
    }

    pub fn add_store(&mut self, name: &str) -> DBResult<()> {
        if self.store.contains_key(name) {
            return Err(DBERROR::DataAlreadyExist);
        }
        self.store.insert(name.to_string(), Store::new());
        Ok(())
    }

    pub fn add_custom(&mut self, name: &str) -> DBResult<()> {
        if self.custom.contains_key(name) {
            return Err(DBERROR::DataAlreadyExist);
        }
        self.custom.insert(name.to_string(), CustomComponent::new());
        Ok(())
    }

    pub fn get_resource(&self, name: &str) -> DBResult<&Resource> {
        match self.resource.get(name) {
            Some(r) => Ok(r),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_container(&self, name: &str) -> DBResult<&Container> {
        match self.container.get(name) {
            Some(c) => Ok(c),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_store(&self, name: &str) -> DBResult<&Store> {
        match self.store.get(name) {
            Some(s) => Ok(s),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_custom(&self, name: &str) -> DBResult<&CustomComponent> {
        match self.custom.get(name) {
            Some(c) => Ok(c),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_mut_resource(&mut self, name: &str) -> DBResult<&mut Resource> {
        match self.resource.get_mut(name) {
            Some(r) => Ok(r),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_mut_container(&mut self, name: &str) -> DBResult<&mut Container> {
        match self.container.get_mut(name) {
            Some(c) => Ok(c),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_mut_store(&mut self, name: &str) -> DBResult<&mut Store> {
        match self.store.get_mut(name) {
            Some(s) => Ok(s),
            None => Err(DBERROR::DataNotExist),
        }
    }

    pub fn get_mut_custom(&mut self, name: &str) -> DBResult<&mut CustomComponent> {
        match self.custom.get_mut(name) {
            Some(c) => Ok(c),
            None => Err(DBERROR::DataNotExist),
        }
    }
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            user_time: Vec::new(),
            enter_time: Vec::new(),
            leave_time: Vec::new(),
            default_data: DefaultData {
                datatype: HashMap::new(),
            },
            res_info: ResInfo {
                avg_time: BigNumber::Int(0),
                no_of_users: 0,
                leave: BigNumber::Int(0),
                rawstore: true,
                analysis: true,
            },
        }
    }

    pub fn add_user_time(
        &mut self,
        user: &str,
        queue_join_time: BigNumber,
        queue_length: BigNumber,
    ) -> DBResult<()> {
        self.user_time
            .push((user.to_string(), queue_join_time, queue_length));
        Ok(())
    }

    pub fn add_enter_time(&mut self, user: &str, enter_time: BigNumber) -> DBResult<()> {
        self.enter_time.push((user.to_string(), enter_time));
        Ok(())
    }

    pub fn add_leave_time(&mut self, user: &str, leave_time: BigNumber) -> DBResult<()> {
        self.leave_time.push((user.to_string(), leave_time));
        Ok(())
    }
}

impl Container {
    pub fn new() -> Container {
        Container {
            put_time: Vec::new(),
            get_time: Vec::new(),
            default_data: DefaultData {
                datatype: HashMap::new(),
            },
            cont_info: ContInfo {
                top_get_amount: BigNumber::Int(0),
                top_put_amount: BigNumber::Int(0),
                clear_times: 0,
                rawstore: true,
                analysis: true,
            },
        }
    }

    pub fn add_put_time(
        &mut self,
        put_time: BigNumber,
        put_amount: BigNumber,
        total_amount: BigNumber,
        user: &str,
    ) -> DBResult<()> {
        self.put_time
            .push((put_time, put_amount, total_amount, user.to_string()));
        Ok(())
    }

    pub fn add_get_time(
        &mut self,
        get_time: BigNumber,
        get_amount: BigNumber,
        total_amount: BigNumber,
        user: &str,
    ) -> DBResult<()> {
        self.get_time
            .push((get_time, get_amount, total_amount, user.to_string()));
        Ok(())
    }
}

impl Store {
    pub fn new() -> Store {
        Store {
            put_time: Vec::new(),
            get_time: Vec::new(),
            default_data: DefaultData {
                datatype: HashMap::new(),
            },
            store_info: StoreInfo {
                top_get_amount: BigNumber::Int(0),
                top_put_amount: BigNumber::Int(0),
                clear_times: 0,
                rawstore: true,
                analysis: true,
            },
        }
    }

    pub fn add_put_time(
        &mut self,
        put_time: BigNumber,
        put_amount: BigNumber,
        total_amount: BigNumber,
        user: &str,
    ) -> DBResult<()> {
        self.put_time
            .push((put_time, put_amount, total_amount, user.to_string()));
        Ok(())
    }

    pub fn add_get_time(
        &mut self,
        get_time: BigNumber,
        get_amount: BigNumber,
        total_amount: BigNumber,
        user: &str,
    ) -> DBResult<()> {
        self.get_time
            .push((get_time, get_amount, total_amount, user.to_string()));
        Ok(())
    }
}

impl CustomComponent {
    pub fn new() -> CustomComponent {
        CustomComponent {
            des: String::new(),
            default_data: DefaultData {
                datatype: HashMap::new(),
            },
        }
    }

    pub fn default_data(&mut self, key: &str, datatype: Datatype) -> () {
        self.default_data.datatype.insert(key.to_string(), datatype);
    }
}

impl BigNumber {
    pub fn covnvert_to_int(&self) -> i64 {
        match self {
            BigNumber::Int(i) => *i,
            BigNumber::Float(f) => *f as i64,
        }
    }

    pub fn covnvert_to_float(&self) -> f64 {
        match self {
            BigNumber::Int(i) => *i as f64,
            BigNumber::Float(f) => *f,
        }
    }
}
