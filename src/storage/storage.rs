pub trait Storage {
    fn get(&self) -> f32;
}

pub struct Mock {

}

impl Storage for Mock {
    fn get(&self) -> f32 {
        12.1
    }
}

impl Mock {
    pub fn new() -> Mock {
        Mock {}
    }
}
