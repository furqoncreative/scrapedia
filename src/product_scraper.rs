use std::error::Error;
use headless_chrome::Browser;
use crate::product::TokopediaProduct;

pub async fn scrape_product(query: &str) -> Result<Vec<TokopediaProduct>, Box<dyn Error>> {
    let mut tokopedia_products: Vec<TokopediaProduct> = Vec::new();

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100 Safari/537.36";
    tab.set_user_agent(user_agent, None, None)
        .expect("Unable to set user agent");

    let url = format!("https://www.tokopedia.com/search?st=&q={query}");
    tab.navigate_to(&url).unwrap();
    tab.wait_until_navigated().expect("Page failed to loaded");

    let products = tab.wait_for_elements("div[data-testid=master-product-card]")?;

    for product in products {
        let html_product = scraper::Html::parse_document(&product.get_content().unwrap());

        let product_name = html_product
            .select(&scraper::Selector::parse(".prd_link-product-name").unwrap())
            .next()
            .map(|store| store.text().collect::<String>());

        let rating = html_product
            .select(&scraper::Selector::parse(".prd_rating-average-text").unwrap())
            .next()
            .map(|store| store.text().collect::<String>());

        let store_name = html_product
            .select(&scraper::Selector::parse(".prd_link-shop-name").unwrap())
            .next()
            .map(|store| store.text().collect::<String>());

        let store_location = html_product
            .select(&scraper::Selector::parse(".prd_link-shop-loc").unwrap())
            .next()
            .map(|store| store.text().collect::<String>());

        let purchase_amount = html_product
            .select(&scraper::Selector::parse(".prd_label-integrity").unwrap())
            .next()
            .map(|store| store.text().collect::<String>());

        let discount_info = html_product
            .select(&scraper::Selector::parse(".prd_badge-product-discount").unwrap())
            .next()
            .map(|store| store.text().collect::<String>());

        let product_image_url = html_product
            .select(&scraper::Selector::parse("img[data-testid=imgSRPProdMain]").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);

        let tokopedia_product = TokopediaProduct {
            product_name,
            rating,
            store_name,
            store_location,
            purchase_amount,
            discount_info,
            product_image_url,
        };

        tokopedia_products.push(tokopedia_product)
    }

    Ok(tokopedia_products)
}
