use std::fs;
use anyhow::Result;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptions};
use std::time::{SystemTime, UNIX_EPOCH};
use rayon::prelude::*;

static SNAPSHOTS_FOLDER: &str = "snapshots";
static SNAPSHOTS_QUALITY: u32 = 75;
static SNAPSHOTS_EXTENSION: &str = ".jpg";
static SNAPSHOT_WINDOW_SIZE: Option<(u32, u32)> = Some((1920, 1080));

static LIST_OF_WEBSITES: &str = "https://raw.githubusercontent.com/Athanaze/news-websites/main/list";

fn timestamp_now() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs_f64() as u64
}

fn timestamp_now_string() -> String {
    timestamp_now().to_string()
}
// extension : .png, .jpg, etc... INCLUDE THE .
fn generate_filename(url: String, extension: &str) ->String {
    use sha256::digest;
    let first_part =  &(SNAPSHOTS_FOLDER.to_owned()+"/"+ &*digest(url) +"/");
    match fs::create_dir_all(first_part){
        Ok(_) => first_part.to_owned()+&*timestamp_now_string()+ &*extension,
        Err(_) => "error".to_string()
    }
}

fn main() -> Result<()> {

    fs::create_dir_all(SNAPSHOTS_FOLDER)?;
    let body = ureq::get(LIST_OF_WEBSITES).call()?.into_string()?;

    let mut v_lines: Vec<String> = Vec::new();
    for url in body.lines(){
        v_lines.push(url.to_string())
    }

    let results = v_lines.par_iter().map(|url| {
        match Browser::new(LaunchOptions::default_builder().window_size(SNAPSHOT_WINDOW_SIZE).sandbox(true).build().expect("Couldn't find appropriate Chrome binary.")) {
            Ok(browser) => {
                match browser.wait_for_initial_tab() {
                    Ok(tab) => {
                        match tab.navigate_to(&url.clone()) {
                            Ok(r) => {
                                match r.wait_until_navigated() {
                                    Ok(r) => {
                                        match r.capture_screenshot(CaptureScreenshotFormatOption::Jpeg, Some(SNAPSHOTS_QUALITY), None, true) {
                                            Ok(jpeg_data) => {
                                                match fs::write(generate_filename(url.clone(), SNAPSHOTS_EXTENSION), &jpeg_data) {
                                                    Ok(_) => {
                                                        match r.print_to_pdf(None) {
                                                            Ok(pdf_data) => {
                                                                match fs::write(generate_filename(url.clone(), ".pdf"), &pdf_data) {
                                                                    Ok(_) => true,
                                                                    Err(_) => {println!("error write pdf");false }
                                                                }
                                                            },
                                                            Err(_) => {println!("error print_to_pdf");false }
                                                        }
                                                    },
                                                    Err(_) => {println!("error write");false }
                                                }
                                            },
                                            Err(_) => {println!("error capture_screenshot");false}
                                        }
                                    },
                                    Err(_) => {println!("error wait_until_navigated");false}
                                }
                            },
                            Err(e) => {println!("error navigate_to: {}", e);false}
                        }
                    },
                    Err(e) => {println!("error wait for init tab : {}", e);false}
                }
            },
            Err(e) => {println!("error browser::new : {}", e);false}
        }
    });

    let n_failures = results.filter(|&a: &bool|{ !a }).count();

    println!("# of failure : {}", n_failures);
    Ok(())
}