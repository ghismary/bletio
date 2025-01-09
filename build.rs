use std::collections::HashMap;
use std::env;
use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Error;
use std::path::Path;

use convert_case::{Case, Casing};
use serde::Deserialize;

#[derive(Debug)]
enum BuildRsError {
    EnvironmentVar,
    IO,
    Yaml,
}

impl From<VarError> for BuildRsError {
    fn from(_value: VarError) -> Self {
        BuildRsError::EnvironmentVar
    }
}

impl From<Error> for BuildRsError {
    fn from(_value: Error) -> Self {
        BuildRsError::IO
    }
}

impl From<serde_yml::Error> for BuildRsError {
    fn from(_value: serde_yml::Error) -> Self {
        BuildRsError::Yaml
    }
}

fn main() -> Result<(), BuildRsError> {
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR")?;
    let out_path = Path::new(&out_dir);

    generate_common_data_types(out_path)?;
    generate_services_assigned_numbers(out_path)?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct AdType {
    value: u8,
    name: String,
    reference: String,
}

impl AdType {
    fn normalized_name(&self) -> String {
        self.name
            .replace("16-bit Service or Service Class UUIDs", "ServiceUuid16")
            .replace("32-bit Service or Service Class UUIDs", "ServiceUuid32")
            .replace("128-bit Service or Service Class UUIDs", "ServiceUuid128")
            .replace(
                "16-bit Service Solicitation UUIDs",
                "SolicitationServiceUuid16",
            )
            .replace(
                "32-bit Service Solicitation UUIDs",
                "SolicitationServiceUuid32",
            )
            .replace(
                "128-bit Service Solicitation UUIDs",
                "SolicitationServiceUuid128",
            )
            .replace("16-bit UUID", "Uuid16")
            .replace("32-bit UUID", "Uuid32")
            .replace("128-bit UUID", "Uuid128")
            .replace("3D", "ThreeDimensional")
            .split(' ')
            .map(|s| s.to_case(Case::Pascal))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Display for AdType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\t/// {} AdType - See {}\n\t{} = {:#04X},",
            self.name,
            self.reference,
            self.normalized_name(),
            self.value
        ))
    }
}

fn generate_common_data_types(out_path: &Path) -> Result<(), BuildRsError> {
    println!("cargo:rerun-if-changed=spec-files/ad_types.yaml");

    let source_path = Path::new("spec-files/ad_types.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let yaml: HashMap<String, Vec<AdType>> = serde_yml::from_str(&yaml_str)?;
    let types_strs: Vec<String> = yaml["ad_types"]
        .iter()
        .filter(|item| item.name != "Device ID")
        .map(|item| item.to_string())
        .collect();
    let types_str = types_strs.join("\n");
    println!("{:?}", types_str);

    let dest_path = out_path.join("ad_types.rs");
    fs::write(
        dest_path,
        format!(
            r#"
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth Common Data Types.
pub(crate) enum AdType {{
{}
}}
"#,
            types_str
        ),
    )?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Uuid {
    uuid: u16,
    name: String,
    id: String,
}

impl Uuid {
    fn normalized_name(&self) -> String {
        self.name
            .split(' ')
            .map(|s| s.to_case(Case::Pascal))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\t/// {} Service - {}\n\t{} = {:#06X},",
            self.name,
            self.id,
            self.normalized_name(),
            self.uuid
        ))
    }
}

fn generate_services_assigned_numbers(out_path: &Path) -> Result<(), BuildRsError> {
    println!("cargo:rerun-if-changed=spec-files/service_uuids.yaml");

    let source_path = Path::new("spec-files/service_uuids.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let yaml: HashMap<String, Vec<Uuid>> = serde_yml::from_str(&yaml_str)?;
    let uuids_strs: Vec<String> = yaml["uuids"].iter().map(|item| item.to_string()).collect();
    let uuids_str = uuids_strs.join("\n");

    let dest_path = out_path.join("services.rs");
    fs::write(
        dest_path,
        format!(
            r#"
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth GATT services.
pub enum Service {{
{}
}}
"#,
            uuids_str
        ),
    )?;

    Ok(())
}
