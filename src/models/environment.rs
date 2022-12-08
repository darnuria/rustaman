use std::vec::Vec;



use super::status::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Environment {
    id: usize,
    name: String,
    payload: String,
    status: Status,
}

impl Environment {
    pub fn new(id: usize, name: &str, payload: &str) -> Self {
        Environment {
            id,
            name: name.to_owned(),
            payload: payload.to_owned(),
            status: Status::Active,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn payload(&self) -> &str {
        self.payload.as_str()
    }
    pub fn set_payload(&mut self, payload: &str) {
        self.payload = payload.to_owned()
    }

    pub fn active(&self) -> bool {
        match self.status {
            Status::Active => true,
            _ => false,
        }
    }

    pub fn soft_delete(&mut self) {
        self.status = Status::Deleted;
    }

    pub fn parsed_payload(&self) -> serde_yaml::Result<serde_yaml::Value> {
        let parsed: serde_yaml::Result<serde_yaml::Value> = serde_yaml::from_str(self.payload());
        parsed
    }

    pub fn obfuscated_string(&self) -> Vec<String> {
        let payload = self.parsed_payload();
        let keys: Vec<String> = match payload {
            Ok(ref data) => {
                let obf = data.get("__obfuscated__");
                info!("{:?}", obf);
                match obf {
                    Some(serde_yaml::Value::Sequence(seq)) => seq
                        .iter()
                        .map(|i| {
                            if let serde_yaml::Value::String(ref s) = i {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .collect(),
                    _ => return vec![],
                }
            }
            _ => return vec![],
        };

        let val: Vec<String> = keys
            .iter()
            .map(|k| {
                if let Some(serde_yaml::Value::String(s)) = payload.as_ref().unwrap().get(k) {
                    Some(s)
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().clone())
            .collect();
        val
    }
}

pub type Environments = Vec<Environment>;
