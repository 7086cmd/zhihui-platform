use deduction::Deduction;

enum Position {
  None,
  Register,
  Clerk,
  ViceMinister,
  Minister,
  ViceChairman,
  Chairman
}

enum Duty {
  Deduction,
  Post,
  Radio,
  Volunteer,
}

enum Admin {
  Deduction,
  Post,
  Radio,
  Volunteer,
  Member,
  MemberVolunteer
}

struct UnionRegister {
  pub plan: String,
  pub prize: String,
  pub position: String,
  pub introduce: String,
}

struct Violation {
  reason: String,
  actioner: u32,
  time: String,
}

struct UnionInfo {
  pub position: Position,
  pub department: String,
  pub group: String,
  pub duty: Duty,
  pub admin: Admin
}

pub struct Member {
  pub name: String,
  pub number: u32,
  pub deduction: Vec<Deduction>,
  pub union_info: UnionInfo
}
