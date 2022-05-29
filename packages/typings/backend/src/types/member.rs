extern crate crypto;
use deduction::Deduction;
use post::Post;

use crypto::sha2::Sha512;

pub mod member {
  struct Register {
    pub prize: String,
    pub plan: String,
    pub position: String,
    pub introduce: String,
  }
  struct UnionInfo {
    pub position: String, // "Chairman" | "Vice Chairman" | "Minister" | "Vice Minister" | "Member" | "Register" | "None"
    pub department: String,
    pub duty: Vec<String>,  // "Deduction" | "Post" | "Task" | "Volunteer"
    pub admin: Vec<String>, // "Deduction" | "Post" | "Task" | "Volunteer" | "Member"
    pub regist: Register,
    pub score: u8,
    pub violations: Vec<Violation>,
  }
  struct Violation {
    pub reason: String,
    pub date: String,
    pub actioner: String,
    pub description: String,
  }
  pub struct Member {
    pub name: String,
    pub number: u32,
    pub deduction: Vec<Deduction>,
    pub posts: Vec<Post>,
    pub union_info: UnionInfo,
    password: String, // base64 + sha512
  }
  impl Member {
    fn check_password(&self) -> String {
      let mut hasher = Sha512::new();
      let based = base64::encode(&self.password);
      hasher.input_str(&self.password);
      let result = hasher.result_str();
      result.to_string() == self.password.to_string()
    }
    fn set_password(&mut self, password: String) -> Result((), std::error::Error) {
      let mut hasher = Sha512::new();
      hasher.input_str(&password);
      self.password = hasher.result_str();
      Ok(())
    }
  }
}
