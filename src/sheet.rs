use google_sheets4::{Sheets, api::ValueRange, hyper_rustls, hyper_util};
use tokio::sync::OnceCell;

type SheetsClient =
    Sheets<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>;

pub struct SheetManager {
    client: SheetsClient,
    spreadsheet_id: String,
}

static SHEET: OnceCell<SheetManager> = OnceCell::const_new();

impl SheetManager {
    async fn new(spreadsheet_id: String) -> Self {
        let secret = yup_oauth2::read_application_secret("client_secret.json")
            .await
            .expect("Cannot read client secret");

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .expect("Failed to build authenticator");

        // Pre-authenticate with all required scopes to avoid multiple OAuth prompts
        auth.token(&[
            "https://www.googleapis.com/auth/spreadsheets",
            "https://www.googleapis.com/auth/drive",
            "https://www.googleapis.com/auth/drive.readonly",
        ])
        .await
        .expect("Failed to get initial token");

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .unwrap()
                        .https_or_http()
                        .enable_http1()
                        .build(),
                );

        Self {
            client: Sheets::new(client, auth),
            spreadsheet_id,
        }
    }

    pub async fn read(
        &self,
        range: &str,
    ) -> Result<Vec<Vec<serde_json::Value>>, google_sheets4::Error> {
        let result = self
            .client
            .spreadsheets()
            .values_get(&self.spreadsheet_id, range)
            .doit()
            .await?;

        Ok(result.1.values.unwrap_or_default())
    }

    #[allow(unused)]
    pub async fn write(
        &self,
        range: &str,
        values: Vec<Vec<serde_json::Value>>,
    ) -> Result<(), google_sheets4::Error> {
        let value_range = ValueRange {
            range: Some(range.to_string()),
            values: Some(values),
            ..Default::default()
        };

        self.client
            .spreadsheets()
            .values_update(value_range, &self.spreadsheet_id, range)
            .value_input_option("USER_ENTERED")
            .doit()
            .await?;

        Ok(())
    }

    pub async fn append(
        &self,
        range: &str,
        values: Vec<Vec<serde_json::Value>>,
    ) -> Result<(), google_sheets4::Error> {
        let value_range = ValueRange {
            range: Some(range.to_string()),
            values: Some(values),
            ..Default::default()
        };

        self.client
            .spreadsheets()
            .values_append(value_range, &self.spreadsheet_id, range)
            .value_input_option("USER_ENTERED")
            .insert_data_option("INSERT_ROWS")
            .doit()
            .await?;

        Ok(())
    }

    pub async fn clear(&self, range: &str) -> Result<(), google_sheets4::Error> {
        self.client
            .spreadsheets()
            .values_clear(
                google_sheets4::api::ClearValuesRequest::default(),
                &self.spreadsheet_id,
                range,
            )
            .doit()
            .await?;

        Ok(())
    }
}

pub async fn init_sheet(spreadsheet_id: &str) {
    SHEET
        .get_or_init(|| SheetManager::new(spreadsheet_id.to_string()))
        .await;
}

pub fn sheet() -> &'static SheetManager {
    SHEET
        .get()
        .expect("Sheet not initialized. Call init_sheet() first.")
}

pub async fn get_uma_list() -> Vec<String> {
    let data = sheet().read("Uma!A:A").await.expect("Failed to read");

    data.into_iter()
        .filter_map(|row| row.into_iter().next())
        .filter_map(|val| val.as_str().map(|s| s.to_string()))
        .collect()
}
