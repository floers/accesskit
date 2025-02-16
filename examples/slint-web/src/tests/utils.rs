use anyhow::{anyhow, Context};
use std::process::Stdio;
use thirtyfour::prelude::*;
use tokio::process::Child;

async fn start_geckodriver() -> Child {
    tokio::process::Command::new("killall")
        .args(vec!["geckodriver"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .output()
        .await
        .unwrap();

    tokio::process::Command::new("geckodriver")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
}

pub(super) async fn connect_to_gecko() -> Result<(WebDriver, Child), anyhow::Error> {
    let c = start_geckodriver().await;

    let mut driver = None;
    for _ in 0..4 {
        let mut caps = DesiredCapabilities::firefox();
        caps.set_headless().unwrap();
        match WebDriver::new("http://localhost:4444", caps).await {
            Ok(d) => {
                driver = Some(d);
                break;
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("retry");
    }
    match driver {
        Some(driver) => Ok((driver, c)),
        None => Err(anyhow!("Failed to connect to driver")),
    }
}

pub(super) async fn find_by_tag(
    driver: &WebDriver,
    tag: &str,
) -> anyhow::Result<thirtyfour::prelude::WebElement> {
    let element = driver
        .find(By::Tag(tag))
        .await
        .context(format!("Cannot find <{tag}>"))?;
    Ok(element)
}

pub(super) async fn find_by_aria_label(
    driver: &WebDriver,
    aria_label: &str,
) -> anyhow::Result<thirtyfour::prelude::WebElement> {
    let element = driver
        .find(By::Css(format!("[aria-label='{aria_label}']")))
        .await
        .context(format!("Cannot find [aria-label='{aria_label}']"))?;
    Ok(element)
}

pub(super) async fn click(driver: &WebDriver, e: &thirtyfour::WebElement) {
    let rect = e.rect().await.unwrap();
    driver
        .action_chain()
        .move_to(rect.x as i64, rect.y as i64)
        .click()
        .perform()
        .await
        .unwrap();
}
