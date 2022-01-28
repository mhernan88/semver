pub struct Semver {
    major: Option<i16>,
    minor: Option<i16>,
    patch: Option<i16>,
}

impl Semver {
    pub fn from_string(&mut self, version: String) -> Result<(), &'static str> {
        let split_version_iter = version.split(".");
        let split_version: Vec<&str> = split_version_iter.collect();

        if split_version.len() == 0 {
            return Err("invalid semantic version")
        } else if split_version.len() == 1 {
            let major = split_version[0].parse::<i16>().unwrap();

            self.major = Some(major);
            self.minor = Some(0);
            self.patch = Some(0);
        } else if split_version.len() == 2 {
            let major = split_version[0].parse::<i16>().unwrap();
            let minor = split_version[1].parse::<i16>().unwrap();

            self.major = Some(major);
            self.minor = Some(minor);
            self.patch = Some(0);
        } else if split_version.len() == 3 {
            let major = split_version[0].parse::<i16>().unwrap();
            let minor = split_version[1].parse::<i16>().unwrap();
            let patch = split_version[2].parse::<i16>().unwrap();

            self.major = Some(major);
            self.minor = Some(minor);
            self.patch = Some(patch);
        } else {
            return Err("invalid semantic version")
        }

        Ok(())
    }

    pub fn to_string(&self) {
        let major_opt = self.major;
        let major: i16;
        if major_opt.is_none() {
            major = 0;
        } else {
            major = major_opt.unwrap();
        }

        let minor_opt = self.minor;
        let minor: i16;
        if minor_opt.is_none() {
            minor = 0;
        } else {
            minor = minor_opt.unwrap();
        }

        let patch_opt = self.patch;
        let patch: i16;
        if patch_opt.is_none() {
            patch = 0;
        } else {
            patch = patch_opt.unwrap();
        }

        println!("{}.{}.{}", major.to_string(), minor.to_string(), patch.to_string());
    }

    pub fn increment_major(&mut self) -> Result<(), &'static str> {
        if self.major == None {
            return Err("cannot increment major version of None");
        }

        let new_major = self.major.unwrap() + 1;

        self.major = Some(new_major);
        self.minor = Some(0);
        self.patch = Some(0);

        Ok(())
    }

    pub fn increment_minor(&mut self) -> Result<(), &'static str> {
        if self.major == None {
            return Err("cannot increment minor version with major version of None");
        }
        if self.minor == None {
            return Err("cannot increment minor version of None");
        }

        let new_minor = self.minor.unwrap() + 1;
        self.minor = Some(new_minor);
        self.patch = Some(0);

        Ok(())
    }

    pub fn increment_patch(&mut self) -> Result<(), &'static str> {
        if self.major == None {
            return Err("cannot increment patch version with major version of None");
        }
        if self.minor == None {
            return Err("cannot increment patch version with minor version of None");
        }
        if self.patch == None {
            return Err("cannot increment patch version of None")
        }

        let new_patch = self.patch.unwrap() + 1;
        self.patch = Some(new_patch);

        Ok(())
    }
}

pub fn new(
    major: Option<i16>,
    minor: Option<i16>,
    patch: Option<i16>,
) -> Semver {
    return Semver{
        major,
        minor,
        patch,
    };
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn semver1() {
        let semver = new(None, None, None);
        assert_eq!(semver.major, None);
        assert_eq!(semver.minor, None);
        assert_eq!(semver.patch, None);
    }

    #[test]
    fn semver2() {
        let major: Option<i16> = Some(1);

        let semver = new(major, None, None);
        assert_eq!(semver.major, major);
        assert_eq!(semver.minor, None);
        assert_eq!(semver.patch, None);
    }

    #[test]
    fn semver3() {
        let major: Option<i16> = Some(1);
        let minor: Option<i16> = Some(2);

        let semver = new(major, minor, None);
        assert_eq!(semver.major, major);
        assert_eq!(semver.minor, minor);
        assert_eq!(semver.patch, None);
    }

    #[test]
    fn semver4() {
        let major: Option<i16> = Some(1);
        let minor: Option<i16> = Some(2);
        let patch: Option<i16> = Some(3);

        let semver = new(major, minor, patch);
        assert_eq!(semver.major, major);
        assert_eq!(semver.minor, minor);
        assert_eq!(semver.patch, patch);
    }

    #[test]
    fn semver_from_string1() {
        let mut semver = new(None, None, None);

        let result = semver.from_string(String::from("1"));
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major, Some(1));
        assert_eq!(semver.minor, Some(0));
        assert_eq!(semver.patch, Some(0));
    }

    #[test]
    fn semver_from_string2() {
        let major: Option<i16> = Some(2);
        let minor: Option<i16> = Some(4);
        let patch: Option<i16> = Some(6);

        let mut semver = new(major, minor, patch);

        let result = semver.from_string(String::from("1"));
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major, Some(1));
        assert_eq!(semver.minor, Some(0));
        assert_eq!(semver.patch, Some(0));
    }

    #[test]
    fn semver_from_string3() {
        let mut semver = new(None, None, None);

        let result = semver.from_string(String::from("1.80"));
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major, Some(1));
        assert_eq!(semver.minor, Some(80));
        assert_eq!(semver.patch, Some(0));
    }

    #[test]
    fn semver_from_string4() {
        let major: Option<i16> = Some(2);
        let minor: Option<i16> = Some(4);
        let patch: Option<i16> = Some(6);

        let mut semver = new(major, minor, patch);

        let result = semver.from_string(String::from("1.80"));
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major, Some(1));
        assert_eq!(semver.minor, Some(80));
        assert_eq!(semver.patch, Some(0));
    }

    #[test]
    fn semver_from_string5() {
        let mut semver = new(None, None, None);

        let result = semver.from_string(String::from("1.80.04"));
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major, Some(1));
        assert_eq!(semver.minor, Some(80));
        assert_eq!(semver.patch, Some(4));
    }

    #[test]
    fn semver_from_string6() {
        let major: Option<i16> = Some(2);
        let minor: Option<i16> = Some(4);
        let patch: Option<i16> = Some(6);

        let mut semver = new(major, minor, patch);

        let result = semver.from_string(String::from("1.80.04"));
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major, Some(1));
        assert_eq!(semver.minor, Some(80));
        assert_eq!(semver.patch, Some(4));
    }

    #[test]
    fn semver_increment_major1() {
        let major: Option<i16> = Some(2);
        let minor: Option<i16> = Some(4);
        let patch: Option<i16> = Some(6);

        let mut semver = new(major, minor, patch);

        let result = semver.increment_major();
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major.unwrap(), major.unwrap() + 1);
        assert_eq!(semver.minor.unwrap(), 0);
        assert_eq!(semver.patch.unwrap(), 0);
    }

    #[test]
    fn semver_increment_minor1() {
        let major: Option<i16> = Some(2);
        let minor: Option<i16> = Some(4);
        let patch: Option<i16> = Some(6);

        let mut semver = new(major, minor, patch);

        let result = semver.increment_minor();
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major.unwrap(), major.unwrap());
        assert_eq!(semver.minor.unwrap(), minor.unwrap() + 1);
        assert_eq!(semver.patch.unwrap(), 0);
    }

    #[test]
    fn semver_increment_patch1() {
        let major: Option<i16> = Some(2);
        let minor: Option<i16> = Some(4);
        let patch: Option<i16> = Some(6);

        let mut semver = new(major, minor, patch);

        let result = semver.increment_patch();
        assert_eq!(result, Ok(()));
        assert_eq!(semver.major.unwrap(), major.unwrap());
        assert_eq!(semver.minor.unwrap(), minor.unwrap());
        assert_eq!(semver.patch.unwrap(), patch.unwrap() + 1);
    }
}
