use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Router};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs};
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct HariLibur {
    tanggal: String,    // Original field name in Indonesian
    keterangan: String, // Original field name in Indonesian
}

#[derive(Debug, Serialize)]
struct Holiday {
    date: String,         // Changed from `tanggal` to `date`
    description: String,  // Changed from `keterangan` to `description`
    is_joint_leave: bool, // New field to indicate joint leave
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    transaction_id: String,
    code: i16,
    message: String,
    data: T,
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to port");

    println!("Server started at http://{}", addr);

    let app = Router::new()
        .route("/", get(root))
        .route("/scrape/{year}", get(scrape_handler))
        .route("/libur/{year}", get(get_libur_handler));

    axum::serve(listener, app)
        .await
        .expect("Error serving application");

    println!("Server running on port {}", port);
}

async fn root() -> &'static str {
    "Welcome to the Holiday API"
}

async fn scrape_handler(
    Path(year): Path<i32>,
) -> Result<Json<ApiResponse<Vec<Holiday>>>, (StatusCode, String)> {
    match scraper_data(year).await {
        Ok(data) => {
            let response = ApiResponse {
                transaction_id: Uuid::new_v4().to_string(),
                code: StatusCode::OK.as_u16() as i16,
                message: "Data retrieved successfully".to_string(),
                data,
            };
            Ok(Json(response))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn get_libur_handler(
    Path(year): Path<i32>,
) -> Result<Json<ApiResponse<HashMap<String, Vec<Holiday>>>>, (StatusCode, String)> {
    let filename = format!("data/{}.json", year);

    match fs::read_to_string(&filename) {
        Ok(contents) => {
            let data: Vec<HariLibur> = serde_json::from_str(&contents)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let mut grouped_data: HashMap<String, Vec<Holiday>> = HashMap::new();
            grouped_data.insert("joint_leave".to_string(), Vec::new());
            grouped_data.insert("non_joint_leave".to_string(), Vec::new());

            for holiday in data {
                let is_joint_leave = holiday.keterangan.contains("Cuti Bersama");
                let holiday_entry = Holiday {
                    date: holiday.tanggal,
                    description: holiday.keterangan.clone(),
                    is_joint_leave,
                };

                if is_joint_leave {
                    grouped_data
                        .get_mut("joint_leave")
                        .unwrap()
                        .push(holiday_entry);
                } else {
                    grouped_data
                        .get_mut("non_joint_leave")
                        .unwrap()
                        .push(holiday_entry);
                }
            }

            let response = ApiResponse {
                transaction_id: Uuid::new_v4().to_string(),
                code: StatusCode::OK.as_u16() as i16,
                message: "Data retrieved successfully".to_string(),
                data: grouped_data,
            };

            Ok(Json(response))
        }
        Err(_) => {
            let response = ApiResponse {
                transaction_id: Uuid::new_v4().to_string(),
                code: StatusCode::NOT_FOUND.as_u16() as i16,
                message: "Data not found for the specified year".to_string(),
                data: String::new(),
            };

            Err((
                StatusCode::NOT_FOUND,
                serde_json::to_string(&response).unwrap(),
            ))
        }
    }
}

async fn scraper_data(year: i32) -> Result<Vec<Holiday>, Box<dyn std::error::Error>> {
    let url = format!("https://www.tanggalan.com/{}", year);
    let response = reqwest::get(&url).await?.text().await?;
    let document = Html::parse_document(&response);
    let ul_selector = Selector::parse("article ul").unwrap();
    let mut data: Vec<HariLibur> = Vec::new();

    let month_map: HashMap<&str, &str> = HashMap::from([
        ("januari", "01"),
        ("februari", "02"),
        ("maret", "03"),
        ("april", "04"),
        ("mei", "05"),
        ("juni", "06"),
        ("juli", "07"),
        ("agustus", "08"),
        ("september", "09"),
        ("oktober", "10"),
        ("november", "11"),
        ("desember", "12"),
    ]);

    for ul in document.select(&ul_selector) {
        let year_str = year.to_string();

        let month_str = ul
            .select(&Selector::parse("li a").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default();

        let month = month_str
            .to_lowercase()
            .replace(char::is_numeric, "")
            .trim()
            .to_string();
        let month_code = month_map.get(month.as_str()).unwrap_or(&"");

        let tr_selector = Selector::parse("li:nth-child(4) tbody tr").unwrap();
        for tr in ul.select(&tr_selector) {
            let day = tr
                .select(&Selector::parse("td:first-child").unwrap())
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let description = tr
                .select(&Selector::parse("td:nth-child(2)").unwrap())
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let full_date = format!("{}-{}-{}", year_str, month_code, day);

            data.push(HariLibur {
                tanggal: full_date,
                keterangan: description,
            });
        }
    }

    fs::create_dir_all("data")?;
    let filename = format!("data/{}.json", year);
    let json_data = serde_json::to_string_pretty(&data)?;
    fs::write(&filename, json_data)?;

    println!("File {} successfully created", filename);

    let holidays: Vec<Holiday> = data
        .into_iter()
        .map(|h| Holiday {
            date: h.tanggal,
            description: h.keterangan.clone(),
            is_joint_leave: h.keterangan.contains("Cuti Bersama"),
        })
        .collect();

    Ok(holidays)
}
