pub mod network {
    use reqwest;

    pub async fn get_page_src(page: String, dest: String) -> Result<(), Box<dyn std::error::Error>> {
        let page_src = reqwest::get(&*page).await?.text().await?;
        let page_src = page_src.as_bytes();

        crate::files::files::write_data(page_src, dest).unwrap_or_default();

        Ok(())
    }
}
