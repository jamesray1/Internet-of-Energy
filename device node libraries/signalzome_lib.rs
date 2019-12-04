// This zome is an ORACLE signalling agent zome.
// if is for an agent who needs to reach out to an external source (API) and relay a value into the dht
// agents running this DNA can review and retrieve this value but unless permitted cannot write to it.


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
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError,
};

  use hdk::holochain_persistence_api::{
    cas::content::Address
};
use hdk_proc_macros::zome;

// this is a struct for the current market price data.  This is updated every 5 minutes.
#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct PriceRange {
    price: String,
    author_id: Address,
}

#[zome]
mod spot_signal {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn price_range_entry_def() -> ValidatingEntryType {
        entry!(
            name: "price",
            description: "this is the current price enum as a string",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<PriceRange>| {
                Ok(())
            },
            links: [
                from!(
                   "%agent_id",
                   link_type: "author_price",
                   validation_package: || {
                       hdk::ValidationPackageDefinition::Entry
                   },
                   validation: |_validation_data: hdk::LinkValidationData| {
                       Ok(())
                   }
                )
            ]
        )
    }

    // this function sets the spot price and is used by the device agents
    // function needs to be written to emit a signal when this changes
    // the signal emit is sent to interested agents who will update their connected device
    #[zome_fn("hc_public")]
    pub fn set_price(price: String) -> ZomeApiResult<Address> {
        let signal = PriceRange {
        price,
        author_id: hdk::AGENT_ADDRESS.clone(),
        };
        let agent_address = hdk::AGENT_ADDRESS.clone().into();
        let entry = Entry::App("price".into(), signal.into());
        let address = hdk::commit_entry(&entry)?;
        hdk::link_entries(&agent_address, &address, "author_price", "")?;
        Ok(address)
    }

    // this is the function that sets the spot price for each state every 5 minutes
    #[zome_fn("hc_public")]
    fn get_price(agent_address: Address) -> ZomeApiResult<Vec<PriceRange>> {
        hdk::utils::get_links_and_load_type(
            &agent_address,
            LinkMatch::Exactly("author_price"),
            LinkMatch::Any,
        )
    }

    #[zome_fn("hc_public")]
    fn get_agent_id() -> ZomeApiResult<Address> {
        Ok(hdk::AGENT_ADDRESS.clone())
    }
}
