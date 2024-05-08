#[derive(Debug)]
pub enum Status {
    Visible = 1 << 0,
    Hidden = 1 << 1,
}

#[derive(Debug)]
pub enum StatusType {
    Checked = 1 << 6,
    Optional = 1 << 7,
}

pub struct StatusHelper;

impl StatusHelper {
    pub fn is_hidden(status: u8) -> bool {
        (status & (Status::Hidden as u8)) != 0
    }

    pub fn is_checked(status: u8) -> bool {
        (status & (StatusType::Checked as u8)) != 0
    }

    pub fn is_visible(status: u8) -> bool {
        (status & (Status::Visible as u8)) != 0
    }

    pub fn is_optional(status: u8) -> bool {
        (status & (StatusType::Optional as u8)) != 0
    }

    pub fn toggle_checked(status: u8) -> u8 {
        status ^ (StatusType::Checked as u8)
    }

    pub fn toggle_visibility(status: u8) -> u8 {
        status ^ (Status::Visible as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let status = Status::Visible as u8;
        assert_eq!(StatusHelper::is_visible(status), true);
        assert_eq!(StatusHelper::is_checked(status), false);
    }
}
