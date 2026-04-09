use forgejo_cli::{ftl_format, localization::bundles};

#[test]
fn placeables() {
    let _ = bundles::LOCALE.set(bundles::init_to("en_US"));

    let s = ftl_format!("test-placeables", name = "Tester");
    assert_eq!(
        s,
        "Hello, \u{2068}Tester\u{2069}! You're \u{2068}Tester\u{2069}."
    );
}

#[test]
fn switch() {
    let _ = bundles::LOCALE.set(bundles::init_to("en_US"));

    assert_eq!(
        ftl_format!("test-switch", n = 0),
        "\u{2068}0\u{2069} things"
    );
    assert_eq!(ftl_format!("test-switch", n = 1), "A thing");
    assert_eq!(
        ftl_format!("test-switch", n = 2),
        "\u{2068}2\u{2069} things"
    );
}
