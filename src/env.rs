#[derive(Debug, Clone)]
pub struct Env<T:Clone> {
    data: Vec<(String, T)>,
}

impl<T:Clone> Env<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }
    pub fn push(&self, key: String, val: T) -> Self {
        let mut new_env = self.clone();
        new_env.data.push((key, val));
        new_env
    }
    pub fn lookup(&self, name:&String) -> Result<T, String> {
        for &(ref key, ref val) in self.data.iter().rev() {
            if key == name {
                return Ok(val.clone());
            }
        }
        Err(format!("unbound variable: {}", name))
    }
}


