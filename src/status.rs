#[derive(Debug)]
pub enum Status {
    Visible(StatusOption),
    Hidden(StatusOption),
    VisibleOptional(StatusOption),
    HiddenOptional(StatusOption),
}

#[derive(Debug)]
pub enum StatusOption {
    Default = 0,
    Checked = 1,
}

impl StatusOption {
    pub fn to_number(self) -> u8 {
        match self {
            StatusOption::Default => 0,
            StatusOption::Checked => 1,
        }
    }
}

impl Status {
    pub fn to_number(self) -> u8 {
        match self {
            Status::Visible(status_option) => status_option.to_number(),
            Status::Hidden(status_option) => status_option.to_number() + 64,
            Status::VisibleOptional(status_option) => status_option.to_number() + 128,
            Status::HiddenOptional(status_option) => status_option.to_number() + 192,
        }
    }
}

#[test]
fn test_status() {
    let status = Status::VisibleOptional(StatusOption::Checked);
    println!("{:?}", status.to_number());
}
