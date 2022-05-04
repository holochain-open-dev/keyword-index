//! ## hc_zome_searchs
//!
//! Search zome for any Holochain app.
//!
//! If you need to manage search (nickname, name, avatar, age and other useful personal information)
//! you can directly include this zome in your DNA.
//!
//! Read about how to include both this zome and its frontend module in your application [here](https://holochain-open-dev.github.io/search).
use hdk::prelude::*;

use hc_zome_keyword_index_types::*;

entry_defs![PathEntry::entry_def()];

#[hdk_extern]
pub fn index_hash_by_keywords(input: IndexHashByKeywordsInput) -> ExternResult<()> {
    for keyword in input.indexed_by_keywords {
        if let Some(first_char) = keyword.chars().nth(0) {
            let path = Path::from(format!("{}_{}", input.index_name, first_char));

            path.ensure()?;

            create_link(
                path.path_entry_hash()?.into(),
                input.indexed_hash.clone().into(),
                HdkLinkType::Any,
                LinkTag::new(keyword.as_bytes()),
            )?;
        }
    }

    Ok(())
}

#[hdk_extern]
pub fn search_by_keyword_prefix(
    input: SearchByKeywordInput,
) -> ExternResult<Vec<SearchByKeywordMatch>> {
    if let Some(first_char) = input.keyword_prefix.chars().nth(0) {
        let path = Path::from(format!("{}_{}", input.index_name, first_char));

        let links = get_links(
            path.path_entry_hash()?.into(),
            Some(LinkTag::new(input.keyword_prefix.as_bytes())),
        )?;

        let matches: Vec<SearchByKeywordMatch> = links
            .into_iter()
            .map(|link| {
                let bytes = link.tag.into_inner();

                let keyword = String::from_utf8_lossy(&bytes);

                SearchByKeywordMatch {
                    matching_keyword: String::from(keyword),
                    matched_hash: link.target.into(),
                }
            })
            .collect();

        Ok(matches)
    } else {
        Err(WasmError::Guest(String::from(
            "Cannot search by empty keyword",
        )))
    }
}
