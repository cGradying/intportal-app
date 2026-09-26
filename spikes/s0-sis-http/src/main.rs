//! Spike S0: can the SIS be signed in to and read over plain HTTP, with no
//! browser? Run it yourself: `cargo run -- sis8`. It asks for your own
//! credentials without echoing them and prints only shapes and counts, never
//! the credentials and never a full row.

use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use scraper::{Html, Selector};
use std::io::{self, Write};

fn trusted(url: &reqwest::Url) -> bool {
    url.scheme() == "https"
        && url
            .host_str()
            .is_some_and(|h| h == "pup.edu.ph" || h.ends_with(".pup.edu.ph"))
}

fn ask(label: &str) -> String {
    print!("{label}: ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn sel(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}

/// Every named input, select and textarea in the login form, as the browser
/// would submit them, so hidden tokens ride along untouched.
fn form_fields(doc: &Html) -> (Option<String>, Vec<(String, String)>) {
    let form = doc.select(&sel("form")).find(|f| f.select(&sel("#studno")).next().is_some());
    let Some(form) = form else { return (None, vec![]) };
    let action = form.value().attr("action").map(str::to_string);
    let mut fields = vec![];
    for el in form.select(&sel("input[name], select[name], textarea[name]")) {
        let name = el.value().attr("name").unwrap().to_string();
        let kind = el.value().attr("type").unwrap_or("").to_ascii_lowercase();
        if kind == "button" {
            continue;
        }
        let value = el.value().attr("value").unwrap_or("").to_string();
        fields.push((name, value));
    }
    (action, fields)
}

fn table_shape(html: &str) -> (Vec<String>, usize, bool) {
    let doc = Html::parse_document(html);
    let headers = doc
        .select(&sel("table thead th"))
        .map(|th| th.text().collect::<String>().trim().to_lowercase())
        .collect();
    let rows = doc.select(&sel("table tbody tr")).count();
    let signed_in = doc
        .select(&sel(r#"a[href*="logout" i], a[href*="signout" i], #logout, .logout"#))
        .next()
        .is_some();
    (headers, rows, signed_in)
}

fn main() {
    let host = std::env::args().nth(1).unwrap_or_else(|| "sis8".into());
    let base = format!("https://{host}.pup.edu.ph/student");

    let client = Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148")
        .redirect(Policy::custom(|a| {
            if trusted(a.url()) && a.previous().len() < 10 { a.follow() } else { a.stop() }
        }))
        .build()
        .unwrap();

    let login_url = format!("{base}/");
    let page = client.get(&login_url).send().expect("login page");
    println!("GET /student/ -> {} ({})", page.status(), page.url());
    let body = page.text().unwrap();
    let doc = Html::parse_document(&body);
    let (action, mut fields) = form_fields(&doc);
    println!("login form action: {action:?}");
    println!("form field names: {:?}", fields.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>());
    println!("page mentions captcha: {}", body.to_lowercase().contains("captcha"));
    let js: String = doc.select(&sel("script")).map(|s| s.text().collect::<String>()).collect();
    for needle in ["submit", "studno", "password", "md5", "sha", "btoa", "encrypt"] {
        println!("login page script mentions {needle:?}: {}", js.contains(needle));
    }
    println!("form tag: {}", doc.select(&sel("form")).find(|f| f.select(&sel("#studno")).next().is_some()).map(|f| { let v=f.value(); format!("method={:?} action={:?} onsubmit={:?}", v.attr("method"), v.attr("action"), v.attr("onsubmit")) }).unwrap_or_default());
    if fields.is_empty() {
        println!("RESULT: no login form with #studno in the served HTML. It is probably built by JS, so plain HTTP fails.");
        return;
    }

    let studno = ask("Student number");
    let month = ask("Birth month (number)");
    let day = ask("Birth day");
    let year = ask("Birth year");
    let password = rpassword::prompt_password("Password (hidden): ").unwrap();

    // Match by id-like names the old app fills. Print which ones were found so
    // a renamed field shows up here rather than as a silent failure.
    let wanted = [
        ("studno", studno),
        ("SelectMonth", month),
        ("SelectDay", day),
        ("SelectYear", year),
        ("password", password),
    ];
    for (id, value) in &wanted {
        let el = doc.select(&sel(&format!("#{id}"))).next();
        let name = el.and_then(|e| e.value().attr("name")).unwrap_or(id).to_string();
        match fields.iter_mut().find(|(n, _)| *n == name) {
            Some(f) => f.1 = value.clone(),
            None => fields.push((name.clone(), value.clone())),
        }
        println!("field #{id} -> name {name:?} (in form: {})", el.is_some());
    }

    let post_url = page_url_join(&login_url, action.as_deref());
    assert!(
        trusted(&reqwest::Url::parse(&post_url).unwrap()),
        "form posts outside https *.pup.edu.ph; refusing to send credentials"
    );
    let resp = client
        .post(&post_url)
        .header("Referer", &login_url)
        .form(&fields)
        .send()
        .expect("login POST");
    println!("POST -> {} ({})", resp.status(), resp.url());
    let after = resp.text().unwrap();
    let (_, _, signed_in) = table_shape(&after);
    let still_form = Html::parse_document(&after).select(&sel("#studno")).next().is_some();
    println!("after POST: logout marker {signed_in}, login form still present {still_form}");
    let after_doc = Html::parse_document(&after);
    for m in after_doc.select(&sel(".modal-body, .alert, .error, .invalid-feedback, .text-danger, #msg, .msg")) {
        let t = m.text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() { println!("page message: {t:.160}"); }
    }
    let scripts: String = after_doc.select(&sel("script")).map(|s| s.text().collect::<String>()).collect();
    for needle in ["alert(", "swal", "Swal.fire", "modal('show')", "toastr"] {
        if let Some(i) = scripts.find(needle) {
            let tail: String = scripts[i..].chars().take(140).collect();
            println!("script hint: {}", tail.replace('\n', " "));
        }
    }

    for page in ["schedule", "grades"] {
        let r = client.get(format!("{base}/{page}")).send().expect(page);
        let url = r.url().to_string();
        let html = r.text().unwrap();
        let (headers, rows, _) = table_shape(&html);
        println!("/{page} -> {url}: {rows} rows, headers {headers:?}");
    }
    println!("RESULT: plain HTTP works if /schedule shows rows above. Check that the headers match the old scraper's columns.");
}

fn page_url_join(base: &str, action: Option<&str>) -> String {
    let base = reqwest::Url::parse(base).unwrap();
    match action {
        Some(a) if !a.is_empty() => base.join(a).unwrap().to_string(),
        _ => base.to_string(),
    }
}
