#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
extern crate geojson;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;

use geojson::Feature;

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct MyEntry {
    content: String,
}


// Use newtype idiom for geojson::Feature to implement DefaultJson
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PointOfInterest(Feature);


#[zome]
mod geospatial {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
     fn my_entry_def() -> ValidatingEntryType {
        entry!(
            name: "point_of_interest",
            description: "point of interest data",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<PointOfInterest>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_point_of_interest(point: PointOfInterest) -> ZomeApiResult<Address> {
        let entry = Entry::App("point_of_interest".into(), point.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_point_of_interest(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

}
