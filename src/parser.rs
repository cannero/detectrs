use anyhow::{Result, anyhow};
use scraper::{Html, Selector};

pub fn extract_sidebar(html_string: &str) -> Result<String> {
    let document = Html::parse_document(html_string);
    let selector_sidebar = Selector::parse("div.docs-sidebar-nav").unwrap();
    let selector_link = Selector::parse("li > div > div > a").unwrap();

    if let Some(element) = document.select(&selector_sidebar).next() {
        let mut text = String::new();
        for link in element.select(&selector_link) {
            text += &link.text().collect::<Vec<_>>().join("");
            text += "\n";
        }

        if text.is_empty() {
            Err(anyhow!("No links found."))
        } else {
            Ok(text)
        }
    } else {
        Err(anyhow!("Sidebar selector not found."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sidebar() {
        let valid_html = include_str!("../testdata/version_8.4.5.html");
        let expected_text = "Preface and Legal Notices
Changes in MySQL 8.4.6 (Not yet released, LTS Release)
Changes in MySQL 8.4.5 (2025-04-15, LTS Release)
Changes in MySQL 8.4.4 (2025-01-21, LTS Release)
Changes in MySQL 8.4.3 (2024-10-15, LTS Release)
Changes in MySQL 8.4.2 (2024-07-23, LTS Release)
Changes in MySQL 8.4.1 (2024-07-01, LTS Release)
Changes in MySQL 8.4.0 (2024-04-30, LTS Release)
";
        let sidebar = extract_sidebar(valid_html);
        assert_eq!(sidebar.unwrap(), expected_text);
    }

    #[test]
    fn test_error_page() {
        let error_page = include_str!("../testdata/error_page.html");
        let not_sidebar = extract_sidebar(error_page);
        assert!(not_sidebar.is_err());
    }

    #[test]
    fn test_sidebar_but_no_anchors() {
        let html = r#"<div class="docs-sidebar-nav">
            <a class="docs-icon-home" href="/doc/"><span class="icon-home"></span>Documentation Home</a><hr />
    <ul>
            <li>
            <div><div class="docs-sidebar-nav-icon"><span class="icon-dot"></span></div><div class="docs-sidebar-nav-link">Version 1</div></div>                    </li>
            <li>
            <div><div class="docs-sidebar-nav-icon"><span class="icon-dot"></span></div><div class="docs-sidebar-nav-link">Version 2</div></div>                    </li>
</ul> </div>"#;

        let sidebar = extract_sidebar(html);
        assert!(sidebar.is_err());
    }
}
