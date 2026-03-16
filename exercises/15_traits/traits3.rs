trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}
struct PaidSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

impl Licensed for PaidSoftware {
    fn licensing_info(&self) -> String {
        "Commercial license".to_string()
    }
}

fn main() {
    // You can optionally experiment here.
    let license = PaidSoftware {
        version_number: 1.to_string(),
    }
    .licensing_info();

    println!("{:?}", license)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
