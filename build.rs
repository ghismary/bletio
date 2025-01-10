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

    generate_ad_types(out_path)?;
    generate_company_identifiers(out_path)?;
    generate_service_uuids(out_path)?;

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

fn generate_ad_types(out_path: &Path) -> Result<(), BuildRsError> {
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
struct CompanyIdentifier {
    value: u16,
    name: String,
    #[serde(skip_deserializing)]
    normalized_name: String,
}

impl CompanyIdentifier {
    fn generate_normalized_name(&mut self, map_count: &mut HashMap<String, usize>) {
        let mut name = self.name.clone();
        name = name
            .replace("\"", "")
            .replace(".", " ")
            .replace(",", " ")
            .replace("/", " ")
            .replace("+", " ")
            .replace("|", " ")
            .replace("!", " ")
            .replace("'", " ")
            .replace("(", " ")
            .replace(")", " ")
            .replace("（", " ")
            .replace("）", " ")
            .replace("&", " And ")
            .split(' ')
            .map(|s| s.to_case(Case::Pascal))
            .collect::<Vec<_>>()
            .join("");
        if name.starts_with(|p: char| p.is_ascii_digit()) {
            name = format!("_{name}");
        }
        let count = map_count.get(&name).unwrap_or(&0);
        if *count == 0 {
            map_count.insert(name.clone(), 1);
        } else {
            *map_count.get_mut(&name).unwrap() += 1;
        }
        self.normalized_name = name;
    }

    fn rectify_normalized_name(&mut self, map_count: &HashMap<String, usize>) {
        let count = map_count.get(&self.normalized_name).unwrap_or(&0);
        if *count > 1 {
            self.normalized_name = format!("{}{:04X}", self.normalized_name, self.value);
        }
    }
}

impl Display for CompanyIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\t/// {} ({:#06X})\n\t{} = {:#06X},",
            self.name, self.value, self.normalized_name, self.value
        ))
    }
}

fn generate_company_identifiers(out_path: &Path) -> Result<(), BuildRsError> {
    println!("cargo:rerun-if-changed=spec-files/company_identifiers.yaml");

    let source_path = Path::new("spec-files/company_identifiers.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let mut yaml: HashMap<String, Vec<CompanyIdentifier>> = serde_yml::from_str(&yaml_str)?;
    let mut normalized_names_count_map: HashMap<String, usize> = HashMap::new();
    let mut company_identifiers: Vec<&mut CompanyIdentifier> = yaml
        .get_mut("company_identifiers")
        .unwrap()
        .iter_mut()
        .collect();
    company_identifiers
        .iter_mut()
        .for_each(|item| item.generate_normalized_name(&mut normalized_names_count_map));
    let company_identifiers: Vec<String> = company_identifiers
        .iter_mut()
        .map(|item| {
            item.rectify_normalized_name(&normalized_names_count_map);
            item.to_string()
        })
        .collect();
    let company_identifiers_str = company_identifiers.join("\n");

    let dest_path = out_path.join("company_identifiers.rs");
    fs::write(
        dest_path,
        format!(
            r#"
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
#[non_exhaustive]
/// Assigned numbers for company identifiers defined in
/// [Assigned Numbers, 7.1](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml).
///
/// It is to be used when creating a Manufacturer Specific Data Advertising Structure.
/// See [ManufacturerSpecificDataAdStruct::try_new](crate::advertising::ad_struct::ManufacturerSpecificDataAdStruct::try_new).
pub enum CompanyIdentifier {{
{}
}}
"#,
            company_identifiers_str
        ),
    )?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct ServiceUuid {
    uuid: u16,
    name: String,
    id: String,
}

impl ServiceUuid {
    fn normalized_name(&self) -> String {
        self.name
            .split(' ')
            .map(|s| s.to_case(Case::Pascal))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Display for ServiceUuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\t/// {} Service ({:#06X}) — {}\n\t{} = {:#06X},",
            self.name,
            self.uuid,
            self.id,
            self.normalized_name(),
            self.uuid
        ))
    }
}

fn generate_service_uuids(out_path: &Path) -> Result<(), BuildRsError> {
    println!("cargo:rerun-if-changed=spec-files/service_uuids.yaml");

    let source_path = Path::new("spec-files/service_uuids.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let yaml: HashMap<String, Vec<ServiceUuid>> = serde_yml::from_str(&yaml_str)?;
    let uuids_strs: Vec<String> = yaml["uuids"].iter().map(|item| item.to_string()).collect();
    let uuids_str = uuids_strs.join("\n");

    let dest_path = out_path.join("service_uuids.rs");
    fs::write(
        dest_path,
        format!(
            r#"
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth GATT services defined in
/// [Assigned Numbers, 3.4](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/uuids/service_uuids.yaml).
///
/// It is be used when creating a list of 16-bit Service UUIDs Advertising Structure.
/// See [ServiceUuid16AdStruct::try_new](crate::advertising::ad_struct::ServiceUuid16AdStruct::try_new).
pub enum ServiceUuid {{
{}
}}
"#,
            uuids_str
        ),
    )?;

    Ok(())
}
