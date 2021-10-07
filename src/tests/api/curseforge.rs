use crate::api::curseforge::{get_files, search};
use crate::util::{print_addons, print_files};

#[tokio::test]
async fn test_search() {
    let result = search("jei").await;
    assert!(result.is_ok());

    print_addons(&result.unwrap());
}

#[tokio::test]
async fn test_get_files() {
    let result = get_files(225179).await;
    assert!(result.is_ok());

    print_files(&result.unwrap());
}
