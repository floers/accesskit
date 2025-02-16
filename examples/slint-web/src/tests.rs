use thirtyfour::WebDriver;

mod suite;
mod utils;
pub use suite::TestSuite;

pub(super) async fn run() -> anyhow::Result<TestSuite> {
    let (driver, mut gecko) = utils::connect_to_gecko().await?;

    driver.goto("http://localhost:3000").await?;
    driver
        .set_implicit_wait_timeout(std::time::Duration::from_secs(3))
        .await?;

    let mut suite = TestSuite {
        successes: 0,
        failures: 0,
    };

    println!("Run tests:");

    test_find_button_by_tag(&driver, &mut suite).await;
    test_find_button_by_aria_label(&driver, &mut suite).await;
    test_find_input_by_tag(&driver, &mut suite).await;
    test_find_input_by_aria_label(&driver, &mut suite).await;
    test_find_checkbox_by_aria_label(&driver, &mut suite).await;
    test_checkbox_checked_state(&driver, &mut suite).await;

    gecko.kill().await.unwrap();
    Ok(suite)
}

async fn test_find_button_by_tag(driver: &WebDriver, suite: &mut TestSuite) {
    suite.test("test_find_button_by_tag").await;
    match utils::find_by_tag(&driver, "button").await {
        Ok(_) => suite.success(),
        Err(_) => suite.fail(""),
    }
}

async fn test_find_button_by_aria_label(driver: &WebDriver, suite: &mut TestSuite) {
    suite.test("test_find_button_by_aria_label").await;
    match utils::find_by_aria_label(&driver, "button label").await {
        Ok(_) => suite.success(),
        Err(_) => suite.fail(""),
    }
}

async fn test_find_input_by_tag(driver: &WebDriver, suite: &mut TestSuite) {
    suite.test("test_find_input_by_tag").await;
    match utils::find_by_tag(&driver, "input").await {
        Ok(_) => suite.success(),
        Err(_) => suite.fail(""),
    }
}

async fn test_find_input_by_aria_label(driver: &WebDriver, suite: &mut TestSuite) {
    suite.test("test_find_input_by_aria_label").await;
    match utils::find_by_aria_label(&driver, "line edit label").await {
        Ok(_) => suite.success(),
        Err(_) => suite.fail(""),
    }
}

async fn test_find_checkbox_by_aria_label(driver: &WebDriver, suite: &mut TestSuite) {
    suite.test("test_find_checkbox_by_aria_label").await;
    match utils::find_by_aria_label(&driver, "checkbox label").await {
        Ok(e) => {
            if e.tag_name().await.unwrap_or_default() != "input" {
                suite.fail("checkbox has wrong tag name");
            } else {
                suite.success();
            }
        }
        Err(_) => suite.fail(""),
    }
}

async fn test_checkbox_checked_state(driver: &WebDriver, suite: &mut TestSuite) {
    suite.test("test_checkbox_checked_state").await;
    match utils::find_by_aria_label(&driver, "checkbox label").await {
        Ok(e) => {
            let checked = e
                .attr("aria-checked")
                .await
                .unwrap_or_default()
                .unwrap_or_default();
            if checked != "false" {
                suite.fail("aria-checked is not 'false'");
                return;
            }

            utils::click(driver, &e).await;

            let checked = e
                .attr("aria-checked")
                .await
                .unwrap_or_default()
                .unwrap_or_default();
            if checked != "true" {
                suite.fail("aria-checked is not 'true'");
                return;
            }
            suite.success();
        }
        Err(_) => suite.fail(""),
    }
}
