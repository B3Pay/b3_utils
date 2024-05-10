pub struct Status {
    _flag: u32,
    _type: u32,
}

impl Status {
    const VISIBLE: u32 = 1 << 0;
    const HIDDEN: u32 = 1 << 1;
    const OPTIONAL: u32 = 1 << 16;
    const CHECKED: u32 = 1 << 17;
    const NEW_OPTION: u32 = 1 << 18;

    pub fn new() -> Self {
        Status {
            _flag: Self::VISIBLE,
            _type: 0,
        }
    }

    pub fn visible(&mut self, types: &[u32]) -> u32 {
        self._flag = Self::VISIBLE;
        for &ty in types {
            self._type |= ty;
        }
        self._flag | self._type
    }

    pub fn hidden(&mut self, types: &[u32]) -> u32 {
        self._flag = Self::HIDDEN;
        for &ty in types {
            self._type |= ty;
        }
        self._flag | self._type
    }

    pub fn is_visible(status: u32) -> bool {
        status & Self::VISIBLE != 0
    }

    pub fn is_hidden(status: u32) -> bool {
        status & Self::HIDDEN != 0
    }

    pub fn is_checked(status: u32) -> bool {
        status & Self::CHECKED != 0
    }

    pub fn is_optional(status: u32) -> bool {
        status & Self::OPTIONAL != 0
    }

    pub fn is_new_option(status: u32) -> bool {
        status & Self::NEW_OPTION != 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let status = Status::new().visible(&[1, 2, 3]);
        assert_eq!(Status::is_visible(status), true);
        assert_eq!(Status::is_checked(status), false);
    }
}
