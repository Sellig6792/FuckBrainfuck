#[derive(Debug, PartialEq, PartialOrd)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn parse(string: &str) -> Version {
        let string = String::from(string).replace('v', "");
        let version: Vec<u8> = string
            .split('.')
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        Version {
            major: version[0],
            minor: version[1],
            patch: version[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_from() {
        let version = Version::parse(&String::from("v1.2.3"));
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_from_without_v() {
        let version = Version::parse(&String::from("1.2.3"));
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_eq() {
        let version1 = Version::parse(&String::from("v1.2.3"));
        let version2 = Version::parse(&String::from("v1.2.3"));
        assert_eq!(version1, version2);
    }

    #[test]
    fn test_version_ne() {
        let version1 = Version::parse(&String::from("v1.2.3"));
        let version2 = Version::parse(&String::from("v1.2.4"));
        assert_ne!(version1, version2);
    }

    #[test]
    fn test_version_lt_patch() {
        let version1 = Version::parse(&String::from("v1.2.3"));
        let version2 = Version::parse(&String::from("v1.2.4"));
        assert!(version1 < version2);
    }

    #[test]
    fn test_version_lt_minor() {
        let version1 = Version::parse(&String::from("v1.2.3"));
        let version2 = Version::parse(&String::from("v1.3.0"));
        assert!(version1 < version2);
    }

    #[test]
    fn test_version_lt_major() {
        let version1 = Version::parse(&String::from("v1.2.3"));
        let version2 = Version::parse(&String::from("v2.0.0"));
        assert!(version1 < version2);
    }

    #[test]
    fn test_version_lt_major_patch() {
        let version1 = Version::parse(&String::from("v1.2.3"));
        let version2 = Version::parse(&String::from("v2.0.1"));
        assert!(version1 < version2);
    }

    #[test]
    fn test_version_lt_major_minor() {
        let version1 = Version::parse(&String::from("v1.2.3"));
        let version2 = Version::parse(&String::from("v2.1.0"));
        assert!(version1 < version2);
    }

    #[test]
    fn test_version_gt_patch() {
        let version1 = Version::parse(&String::from("v1.2.4"));
        let version2 = Version::parse(&String::from("v1.2.3"));
        assert!(version1 > version2);
    }

    #[test]
    fn test_version_gt_minor() {
        let version1 = Version::parse(&String::from("v1.3.0"));
        let version2 = Version::parse(&String::from("v1.2.3"));
        assert!(version1 > version2);
    }

    #[test]
    fn test_version_gt_major() {
        let version1 = Version::parse(&String::from("v2.0.0"));
        let version2 = Version::parse(&String::from("v1.2.3"));
        assert!(version1 > version2);
    }

    #[test]
    fn test_version_gt_major_patch() {
        let version1 = Version::parse(&String::from("v2.0.1"));
        let version2 = Version::parse(&String::from("v1.2.3"));
        assert!(version1 > version2);
    }

    #[test]
    fn test_version_gt_major_minor() {
        let version1 = Version::parse(&String::from("v2.1.0"));
        let version2 = Version::parse(&String::from("v1.2.3"));
        assert!(version1 > version2);
    }

    #[test]
    fn test_version_gt_major_minor_patch() {
        let version1 = Version::parse(&String::from("v2.1.1"));
        let version2 = Version::parse(&String::from("v1.2.3"));
        assert!(version1 > version2);
    }

    #[test]
    fn test_version_lt_major_gt_minor() {
        let version1 = Version::parse(&String::from("v1.3.0"));
        let version2 = Version::parse(&String::from("v2.2.0"));
        assert!(version1 < version2);
    }
}
