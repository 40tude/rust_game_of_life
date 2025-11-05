use headless_chrome::Browser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Duration; //Never used. For the fun :-)

const BASE_URL: &str = "https://conwaylife.com";
const NB_FILES_FOR_TEST: Option<usize> = None; // Some(5) = limit to n files, None = all files
const OUT_DIR: &str = "rle";
const PATTERNS_LIST: &str = "patterns_list.txt";
const NAVIGATION_TIMEOUT_SECS: u64 = 5;
const DOWNLOAD_PAUSE_BASE_MS: u64 = 2000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_patterns_list()?;

    if !Path::new(OUT_DIR).exists() {
        fs::create_dir(OUT_DIR)?;
        println!("Directory '{}' created", OUT_DIR);
    }

    let (success, failed) = download_the_list(OUT_DIR, PATTERNS_LIST)?;

    println!("\nRESULTS :");
    println!("\tSuccess : {}", success);
    println!("\tFailures: {}", failed);
    println!("\tTotal   : {}", success + failed);

    Ok(())
}

fn get_patterns_list() -> Result<(), Box<dyn std::error::Error>> {
    println!("Using headless browser to bypass Cloudflare...");

    let browser = headless_chrome::Browser::default()?;
    let tab = browser.new_tab()?;

    // Reach the page
    tab.navigate_to(&format!("{}{}", BASE_URL, "/patterns/"))?;

    // Wait while the page is loading
    println!("Waiting for page to load...");
    std::thread::sleep(Duration::from_secs(NAVIGATION_TIMEOUT_SECS));

    // Get the HTML
    let html_result = tab.evaluate("document.documentElement.outerHTML", false)?;
    let html_content = html_result.value.and_then(|v| v.as_str().map(|s| s.to_string()));

    let html_content = match html_content {
        Some(html) => html,
        None => return Err("Failed to get HTML content".into()),
    };

    // Parse HTML with scraper to extract the links
    let document = scraper::Html::parse_document(&html_content);
    let selector = scraper::Selector::parse("a[href]").unwrap();

    let mut all_links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href")
            && href.ends_with(".rle")
        {
            // Build complete URL
            let full_url = if href.starts_with("/") {
                format!("{}{}", BASE_URL, href)
            } else if href.starts_with("http") {
                href.to_string()
            } else {
                format!("{}/patterns/{}", BASE_URL, href)
            };
            all_links.push(full_url);
        }
    }

    // Sort and deduplicate
    all_links.sort();
    all_links.dedup();

    if all_links.is_empty() {
        println!("No .rle links found in the HTML");
        println!("HTML content length: {} characters", html_content.len());
        return Err("No .rle links found".into());
    }

    println!("Found {} .rle files", all_links.len());

    // Save in file
    let mut file = File::create(PATTERNS_LIST)?;
    for link in &all_links {
        writeln!(file, "{}", link)?;
    }

    println!("Links saved to: {} ", PATTERNS_LIST);

    // Print the first and last 5 names
    println!("First 5 files:");
    for link in all_links.iter().take(5) {
        let filename = link.split('/').next_back().unwrap_or("unknown");
        println!("  - {}", filename);
    }

    println!("Last 5 files:");
    for link in all_links.iter().rev().take(5) {
        let filename = link.split('/').next_back().unwrap_or("unknown");
        println!("  - {}", filename);
    }

    Ok(())
}

fn download_the_list(output_dir: &str, list_file: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let file = File::open(list_file)?;
    let reader = io::BufReader::new(file);
    let urls: Vec<String> = reader.lines().map_while(Result::ok).collect();

    let total_to_download = NB_FILES_FOR_TEST.unwrap_or(urls.len());
    println!("Downloading {} files (out of {} available)...", total_to_download, urls.len());

    let progress_bar = ProgressBar::new(total_to_download as u64);
    progress_bar.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}").unwrap());

    let browser = Browser::default()?;
    let mut success_count = 0;
    let mut fail_count = 0;

    let urls_to_process = match NB_FILES_FOR_TEST {
        Some(n) => urls.iter().take(n).collect::<Vec<_>>(),
        None => urls.iter().collect::<Vec<_>>(),
    };

    for (index, url) in urls_to_process.iter().enumerate() {
        let filename = match url.split('/').next_back() {
            Some(name) => name,
            None => {
                eprintln!("[{}/{}] URL invalid: {}", index + 1, total_to_download, url);
                fail_count += 1;
                continue;
            }
        };

        let output_path = format!("{}/{}", output_dir, filename);

        progress_bar.set_message(filename.to_string());

        if Path::new(&output_path).exists() {
            println!("[{}/{}] {} already exists", index + 1, total_to_download, filename);
            success_count += 1;
            progress_bar.inc(1);
            continue;
        }

        // println!("[{}/{}] Downloading {}...", index + 1, total_to_download, filename);

        match download_with_browser(&browser, url, &output_path) {
            Ok(true) => {
                // println!("[{}/{}] ✓ Saved: {}", index + 1, total_to_download, filename);
                success_count += 1;
            }
            Ok(false) => {
                // println!("[{}/{}] - Content not found: {}", index + 1, total_to_download, filename);
                fail_count += 1;
            }
            Err(_) => {
                // eprintln!("[{}/{}] ✗ Error: {}", index + 1, total_to_download, e);
                fail_count += 1;
            }
        }

        // Progressive delay (longer at the end)
        let pause_ms = DOWNLOAD_PAUSE_BASE_MS + (index as u64 % 10) * 100; // from 2 to 3 sec
        std::thread::sleep(Duration::from_millis(pause_ms));

        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Done!");

    Ok((success_count, fail_count))
}

fn download_with_browser(browser: &Browser, url: &str, output_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let tab = browser.new_tab()?;

    // RAII so that the browser tab is properly cleared
    struct TabGuard<'a>(&'a headless_chrome::Tab);
    impl Drop for TabGuard<'_> {
        fn drop(&mut self) {
            let _ = self.0.close(true);
        }
    }
    let _guard = TabGuard(&tab); // when _guard goes out of scope the tab is close

    tab.navigate_to(url)?;
    std::thread::sleep(Duration::from_secs(NAVIGATION_TIMEOUT_SECS));

    let html_result = tab.evaluate("document.documentElement.outerHTML", false)?;

    let html_content = if let Some(value) = html_result.value {
        if value.is_string() { value.as_str().map(|s| s.to_string()) } else { None }
    } else {
        None
    };

    if let Some(html) = &html_content
        && (html.contains("Just a moment") || html.contains("cloudflare"))
    {
        // Save HTML for analysis
        // let debug_path = format!("{}.debug.html", output_path);
        // let mut debug_file = File::create(&debug_path)?;
        // debug_file.write_all(html.as_bytes())?;
        eprintln!("⚠️  Cloudflare challenge detected for: {}", url);
        return Err("Cloudflare protection detected".into());
    }

    let extraction_scripts = [
        r#"document.querySelector('pre')?.textContent || ''"#,
        r#"Array.from(document.querySelectorAll('pre')).map(p => p.textContent).find(t => t && t.includes('x =')) || ''"#,
        r#"Array.from(document.querySelectorAll('*')).find(el => { const t = el.textContent; return t && t.includes('x =') && t.includes('y =') && t.includes('rule =') })?.textContent || ''"#,
        r#"document.querySelector('textarea')?.value || ''"#,
        r#"document.querySelector('code')?.textContent || ''"#,
    ];

    for script in &extraction_scripts {
        let result = tab.evaluate(script, false)?;

        if let Some(content_value) = result.value
            && content_value.is_string()
            && let Some(rle_content) = content_value.as_str()
        {
            let trimmed = rle_content.trim();
            if !trimmed.is_empty() && is_valid_rle_content(trimmed) {
                let mut file = File::create(output_path)?;
                file.write_all(trimmed.as_bytes())?;
                return Ok(true);
            }
        }
    }

    Ok(false)
}

// TODO: can be improved
// Make sense to check that way here because we "know" .rle patterns files on BASE_URL are well formed
// This may not be the case somewhere else
fn is_valid_rle_content(content: &str) -> bool {
    let content = content.trim();

    content.contains("x =") && content.contains("y =") && (content.contains("rule") || content.contains('o') || content.contains('b'))
        || content.starts_with("#N")
        || content.starts_with("#C")
        || (content.contains('!') && content.lines().count() >= 3)
}
