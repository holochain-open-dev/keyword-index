use ::fixt::prelude::fixt;

use hc_zome_keyword_index_types::{
    IndexHashByKeywordsInput, SearchByKeywordInput, SearchByKeywordMatch,
};
use hdk::prelude::*;
use holochain::test_utils::consistency_10s;
use holochain::{conductor::config::ConductorConfig, sweettest::*};

#[tokio::test(flavor = "multi_thread")]
async fn index_and_search() {
    // Use prebuilt DNA file
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../workdir/search-demo.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    // Set up conductors
    let mut conductors = SweetConductorBatch::from_config(2, ConductorConfig::default()).await;
    let apps = conductors.setup_app("search-demo", &[dna]).await.unwrap();
    conductors.exchange_peer_info().await;

    let ((alice,), (bobbo,)) = apps.into_tuples();

    let alice_zome = alice.zome("keyword_index");
    let bob_zome = bobbo.zome("keyword_index");

    // Query: "ho"

    // "holochain" -> "ukhuasflkjasdfjlf" (hash of hashtag holochain)
    // "holo" -> "ukhjñljfñlkjasdf" (hash of hashtag holo)

    // "this is a the cool note about holochain" -> "uHash3"
    // call("index_hash_by_keywords", {
    //  "indexed_hash": "uHash3",
    //  "indexed_by_keywords": vec!["this", "is", "a", "the", "cool", "note", "about", "holochain"]
    //}

    /*
    * Link: PATH ENTRY ->|TAG| TARGET HASH
    *
    * MEOW_h ->|holo| uHash3
    * MEOW_h ->|holochain| uHash3

    * NOTES_t ->|this| uHash3
    * NOTES_t ->|the| uHash3
    * NOTES_i ->|is| uHash3
    * NOTES_a ->|a| uHash3
    */

    let fake_hash = fixt!(AnyLinkableHashB64);

    let index_input = IndexHashByKeywordsInput {
        index_name: String::from("my_index"),
        indexed_hash: fake_hash.clone(),
        indexed_by_keywords: vec![String::from("holochain"), String::from("web3")],
    };

    let _r: () = conductors[0]
        .call(&alice_zome, "index_hash_by_keywords", index_input)
        .await;

    consistency_10s(&[&alice, &bobbo]).await;

    let search_input = SearchByKeywordInput {
        index_name: String::from("my_index"),
        keyword_prefix: String::from("ho"),
    };

    let matches: Vec<SearchByKeywordMatch> = conductors[0]
        .call(&bob_zome, "search_by_keyword_prefix", search_input)
        .await;

    assert_eq!(matches[0].matched_hash, fake_hash);
    assert_eq!(matches[0].matching_keyword, String::from("ho"));
}
