use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{arg, command, value_parser};
use convert_case::{Case, Casing};
use git2::Repository;
use serde::Deserialize;

const BLUETOOTH_SIG_URL: &str = "https://bitbucket.org/bluetooth-SIG/public.git";

fn main() -> Result<(), anyhow::Error> {
    let matches = command!()
        .arg(
            arg!([output_folder] "The folder where to output the generated files")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(output_folder) = matches.get_one::<PathBuf>("output_folder") {
        let repository_folder = tempdir::TempDir::new("update-assigned-numbers")?;
        let revision = clone_repository(repository_folder.path())?;

        generate_ad_types(repository_folder.path(), output_folder.as_path(), &revision)?;
        generate_appearance_values(repository_folder.path(), output_folder.as_path(), &revision)?;
        generate_company_identifiers(repository_folder.path(), output_folder.as_path(), &revision)?;
        generate_service_uuids(repository_folder.path(), output_folder.as_path(), &revision)?;
        generate_uri_schemes(repository_folder.path(), output_folder.as_path(), &revision)?;
    }

    Ok(())
}

fn clone_repository(local_path: &Path) -> Result<String, anyhow::Error> {
    let repository = Repository::clone(BLUETOOTH_SIG_URL, local_path)?;
    let obj = repository.revparse_single("main")?;
    Ok(obj.id().to_string())
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

fn generate_ad_types(
    repository_folder: &Path,
    output_folder: &Path,
    revision: &str,
) -> Result<(), anyhow::Error> {
    let source_path = repository_folder.join("assigned_numbers/core/ad_types.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let yaml: HashMap<String, Vec<AdType>> = serde_yml::from_str(&yaml_str)?;
    let types_strs: Vec<String> = yaml["ad_types"]
        .iter()
        .filter(|item| item.name != "Device ID")
        .map(|item| item.to_string())
        .collect();
    let types_str = types_strs.join("\n");

    let dest_path = output_folder.join("ad_types.rs");
    fs::write(
        &dest_path,
        format!(
            r#"//! Assigned numbers for Bluetooth Common Data Types.
//!
//! FILE GENERATED FROM REVISION {} OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
#[allow(dead_code)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth Common Data Types.
pub(crate) enum AdType {{
{}
}}
"#,
            revision, types_str
        ),
    )?;

    rustfmt(dest_path.as_path())?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct AppearanceSubValue {
    value: u8,
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AppearanceValue {
    category: u16,
    name: String,
    subcategory: Option<Vec<AppearanceSubValue>>,
}

#[derive(Debug)]
struct FlattenedAppearanceValue {
    value: u16,
    name: String,
    category_name: String,
    normalized_name: String,
}

impl FlattenedAppearanceValue {
    fn generate_normalized_name(&mut self, map_count: &mut HashMap<String, usize>) {
        let mut name = self.name.clone();
        name = name
            .replace(",", " ")
            .replace("/", " ")
            .replace("(", " ")
            .replace(")", " ")
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
            self.normalized_name = format!(
                "{}{}",
                self.category_name.to_case(Case::Pascal),
                self.normalized_name
            );
        }
    }
}

impl Display for FlattenedAppearanceValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\t/// `{} - {}` Appearance value\n\t{} = {:#06X},",
            self.category_name, self.name, self.normalized_name, self.value
        ))
    }
}

impl From<&AppearanceValue> for Vec<FlattenedAppearanceValue> {
    fn from(value: &AppearanceValue) -> Self {
        let mut result = vec![];
        result.push(FlattenedAppearanceValue {
            value: value.category << 6,
            name: format!("Generic {}", value.name),
            category_name: value.name.clone(),
            normalized_name: Default::default(),
        });
        let mut subvalues = value.subcategory.clone().map_or(vec![], |item| {
            item.iter()
                .map(|item| FlattenedAppearanceValue {
                    value: (value.category << 6) | item.value as u16,
                    name: item.name.clone(),
                    category_name: value.name.clone(),
                    normalized_name: Default::default(),
                })
                .collect()
        });
        result.append(&mut subvalues);
        result
    }
}

fn generate_appearance_values(
    repository_folder: &Path,
    output_folder: &Path,
    revision: &str,
) -> Result<(), anyhow::Error> {
    let source_path = repository_folder.join("assigned_numbers/core/appearance_values.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let yaml: HashMap<String, Vec<AppearanceValue>> = serde_yml::from_str(&yaml_str)?;
    let mut flattened_appearance_values: Vec<FlattenedAppearanceValue> = yaml["appearance_values"]
        .iter()
        .flat_map(|item| {
            let flattened_appearance_values: Vec<FlattenedAppearanceValue> = item.into();
            flattened_appearance_values
        })
        .collect();
    let mut normalized_names_count_map: HashMap<String, usize> = HashMap::new();
    flattened_appearance_values
        .iter_mut()
        .for_each(|item| item.generate_normalized_name(&mut normalized_names_count_map));
    let appearance_strs: Vec<String> = flattened_appearance_values
        .iter_mut()
        .map(|item| {
            item.rectify_normalized_name(&normalized_names_count_map);
            item.to_string()
        })
        .collect();
    let appearance_str = appearance_strs.join("\n");

    let dest_path = output_folder.join("appearance_values.rs");
    fs::write(
        &dest_path,
        format!(
            r#"//! Assigned numbers for appearance values.
//!
//! FILE GENERATED FROM REVISION {} OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
#[allow(dead_code)]
#[non_exhaustive]
/// Assigned numbers for appearance values defined in
/// [Assigned Numbers, 2.6](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/core/appearance_values.yaml).
///
/// It is to be used when creating an Appearance Advertising Structure.
/// See [AppearanceAdStruct::new](crate::advertising::ad_struct::AppearanceAdStruct::new).
pub enum AppearanceValue {{
{}
}}
"#,
            revision, appearance_str
        ),
    )?;

    rustfmt(dest_path.as_path())?;

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

fn generate_company_identifiers(
    repository_folder: &Path,
    output_folder: &Path,
    revision: &str,
) -> Result<(), anyhow::Error> {
    let source_path =
        repository_folder.join("assigned_numbers/company_identifiers/company_identifiers.yaml");
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

    let dest_path = output_folder.join("company_identifiers.rs");
    fs::write(
        &dest_path,
        format!(
            r#"//! Assigned numbers for company identifiers.
//!
//! FILE GENERATED FROM REVISION {} OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
            revision, company_identifiers_str
        ),
    )?;

    rustfmt(dest_path.as_path())?;

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

fn generate_service_uuids(
    repository_folder: &Path,
    output_folder: &Path,
    revision: &str,
) -> Result<(), anyhow::Error> {
    let source_path = repository_folder.join("assigned_numbers/uuids/service_uuids.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let yaml: HashMap<String, Vec<ServiceUuid>> = serde_yml::from_str(&yaml_str)?;
    let uuids_strs: Vec<String> = yaml["uuids"].iter().map(|item| item.to_string()).collect();
    let uuids_str = uuids_strs.join("\n");

    let dest_path = output_folder.join("service_uuids.rs");
    fs::write(
        &dest_path,
        format!(
            r#"//! Assigned numbers for Bluetooth GATT services.
//!
//! FILE GENERATED FROM REVISION {} OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
            revision, uuids_str
        ),
    )?;

    rustfmt(dest_path.as_path())?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct UriScheme {
    value: u16,
    name: String,
}

impl UriScheme {
    fn normalized_name(&self) -> String {
        self.name
            .replace(':', "")
            .replace(['.', '-'], " ")
            .to_case(Case::Pascal)
    }
}

impl Display for UriScheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "\t/// \"{}\" URI scheme\n\t{} = {:#06X},",
            self.name,
            self.normalized_name(),
            self.value
        ))
    }
}

fn generate_uri_schemes(
    repository_folder: &Path,
    output_folder: &Path,
    revision: &str,
) -> Result<(), anyhow::Error> {
    let source_path = repository_folder.join("assigned_numbers/core/uri_schemes.yaml");
    let yaml_str = fs::read_to_string(source_path)?;
    let yaml: HashMap<String, Vec<UriScheme>> = serde_yml::from_str(&yaml_str)?;
    let uri_scheme_strs: Vec<String> = yaml["uri_schemes"]
        .iter()
        .filter(|item| item.value != 0x0001)
        .map(|item| item.to_string())
        .collect();
    let uri_scheme_str = uri_scheme_strs.join("\n");

    let dest_path = output_folder.join("uri_schemes.rs");
    fs::write(
        &dest_path,
        format!(
            r#"//! Assigned numbers for Bluetooth URI schemes.
//!
//! FILE GENERATED FROM REVISION {} OF THE BLUETOOTH SIG REPOSITORY, DO NOT EDIT!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
#[non_exhaustive]
/// Assigned numbers for Bluetooth URI schemes defined in
/// [Assigned Numbers, 2.7](https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/core/uri_schemes.yaml).
///
/// It is be used when creating a Uniform Resource Identifier Advertising Structure.
/// See [UriAdStruct::try_new](crate::advertising::ad_struct::UriAdStruct::try_new).
pub enum ProvisionedUriScheme {{
{}
}}
"#,
            revision, uri_scheme_str
        ),
    )?;

    rustfmt(dest_path.as_path())?;

    Ok(())
}

fn rustfmt(file_path: &Path) -> Result<(), anyhow::Error> {
    Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(file_path)
        .output()?;
    Ok(())
}
