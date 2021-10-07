#[allow(unused_imports)]
use crate::util::game_version_tags_to_modloader;

#[test]
fn test_game_version_tags_to_modloader() {
    let input = vec![
        "Forge".to_string(),
        "1.12.2".to_string(),
        "1.12.1".to_string(),
    ];
    let output = game_version_tags_to_modloader(&input);
    assert_eq!(output, Some("forge".to_string()));
}

#[test]
fn test_game_version_tags_to_modloader_default() {
    let input = vec!["1.12.2".to_string(), "1.12.1".to_string()];
    let output = game_version_tags_to_modloader(&input);
    assert_eq!(output, None);
}
