# API-for-Indonesian-Holidays

This project provides a REST API built with Rust using the `Axum` framework, `Scraper` crate, and `Reqwest` for scraping holiday data from the website "https://www.tanggalan.com/". It allows users to retrieve holiday data in various formats and provides functionality to scrape data for a given year.

The data is parsed from the website, stored in JSON files, and can be accessed through specific routes in the API. The API also distinguishes between joint leave and non-joint leave holidays.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
  - [Root Endpoint](#root-endpoint)
  - [Scrape Holiday Data](#scrape-holiday-data)
  - [Get Holiday Data](#get-holiday-data)
- [Project Structure](#project-structure)
- [Technologies Used](#technologies-used)
- [Environment Variables](#environment-variables)
- [Running the Application](#running-the-application)
- [License](#license)

## Features

- **Scrape holiday data**: The API can scrape holiday data from the website for a given year.
- **Group holidays**: It can group holidays into two categories: `joint_leave` and `non_joint_leave`.
- **Return holiday data in JSON format**: The API provides holiday data in a structured JSON response.
- **Error handling**: Provides informative error messages for issues like missing data or internal server errors.
- **UUID-based transaction IDs**: Each response includes a unique transaction ID for tracking purposes.

## Getting Started

Follow these steps to get the application running on your local machine.

### Prerequisites

- Rust (1.70 or higher)
- Cargo (Rust package manager)
- An active internet connection (for scraping the data)

### Installing Rust

If you haven't installed Rust yet, you can download and install it from [here](https://www.rust-lang.org/tools/install).

### Install Dependencies

Once you have Rust installed, navigate to your project folder and install dependencies by running:

```bash
cargo build
```

This will download and build the necessary dependencies for your project.

## API Endpoints

### Root Endpoint

- **URL**: `/`
- **Method**: `GET`
- **Response**: A welcome message.

#### Example:

```
🎉 Welcome to the Holiday API!

Available endpoints:

- GET /scrape/{year}
    - Scrape holidays for the specific year from an external source.
    - Example: GET /scrape/2023

- GET /libur/{year}
    - Get holidays for a specific year as a list of holidays.
    - Example: GET /libur/2023
    - Returns a list of holidays for the given year.

- GET /libur-grouped/{year}
    - Get holidays for a specific year, grouped by type (e.g., joint leave vs. non-joint leave).
    - Example: GET /libur-grouped/2023
    - Returns holidays grouped by their type.
```

### Scrape Holiday Data

- **URL**: `/scrape/{year}`
- **Method**: `GET`
- **Parameters**: `year` (integer), the year for which to scrape holiday data.
- **Response**: JSON object containing scraped holiday data for the specified year.

#### Example:

```json
{
  "transaction_id": "uuid-string",
  "code": 200,
  "message": "Data retrieved successfully",
  "data": [
    {
      "date": "01-01-2025",
      "description": "New Year's Day",
      "is_joint_leave": true
    },
    {
      "date": "14-04-2025",
      "description": "Good Friday",
      "is_joint_leave": false
    }
  ]
}
```

### Get Holiday Data

- **URL**: `/libur/{year}`
- **Method**: `GET`
- **Parameters**: `year` (integer), the year for which to retrieve the holiday data.
- **Response**: A JSON object containing the holiday data in the specified format.

#### Example:

```json
{
  "transaction_id": "uuid-string",
  "code": 200,
  "message": "Data retrieved successfully",
  "data": [
    {
      "date": "01-01-2025",
      "description": "New Year's Day",
      "is_joint_leave": true
    },
    {
      "date": "14-04-2025",
      "description": "Good Friday",
      "is_joint_leave": false
    }
  ]
}
```

### Get Grouped Holiday Data

- **URL**: `/libur/{year}/grouped`
- **Method**: `GET`
- **Parameters**: `year` (integer), the year for which to retrieve the holiday data, grouped by joint_leave and non_joint_leave.
- **Response**: JSON object containing holiday data grouped by `joint_leave` and `non_joint_leave`.

#### Example:

```json
{
  "transaction_id": "uuid-string",
  "code": 200,
  "message": "Data retrieved successfully",
  "data": {
    "joint_leave": [
      {
        "date": "01-01-2025",
        "description": "New Year's Day",
        "is_joint_leave": true
      }
    ],
    "non_joint_leave": [
      {
        "date": "14-04-2025",
        "description": "Good Friday",
        "is_joint_leave": false
      }
    ]
  }
}
```

## Project Structure

The project consists of the following key files and directories:

```
calendar-indonesia/
├── Cargo.toml
├── README.md
├── .env.example
├── .gitignore
└── src/
    ├── main.rs
    ├── lib.rs
    ├── config/
    │   └── mod.rs
    ├── domain/
    │   ├── mod.rs
    │   ├── entities/
    │   │   ├── mod.rs
    │   │   └── holiday.rs
    │   ├── repositories/
    │   │   ├── mod.rs
    │   │   └── holiday_repository.rs
    │   └── services/
    │       ├── mod.rs
    │       └── holiday_service.rs
    ├── infrastructure/
    │   ├── mod.rs
    │   ├── web/
    │   │   ├── mod.rs
    │   │   ├── handlers/
    │   │   │   ├── mod.rs
    │   │   │   └── holiday_handler.rs
    │   │   └── routes/
    │   │       ├── mod.rs
    │   │       └── holiday_routes.rs
    │   ├── persistence/
    │   │   ├── mod.rs
    │   │   └── file_repository.rs
    │   ├── external/
    │   │   ├── mod.rs
    │   │   └── scraper_service.rs
    │   └── scheduler/
    │       ├── mod.rs
    │       └── holiday_scheduler.rs
    ├── application/
    │   ├── mod.rs
    │   ├── dto/
    │   │   ├── mod.rs
    │   │   ├── holiday_dto.rs
    │   │   └── api_response.rs
    │   └── use_cases/
    │       ├── mod.rs
    │       ├── scrape_holidays.rs
    │       └── get_holidays.rs
    └── shared/
        ├── mod.rs
        ├── errors/
        │   ├── mod.rs
        │   └── app_error.rs
        └── utils/
            ├── mod.rs
            └── date_utils.rs
```

## Technologies Used

- **Rust**: Programming language used for backend development.
- **Axum**: Web framework used to build the API.
- **Scraper**: Web scraping library used to parse holiday data from the website.
- **Serde**: Library for serializing and deserializing data into JSON.
- **Tokio**: Asynchronous runtime used for async operations.
- **Reqwest**: HTTP client for making requests to the website.
- **UUID**: Library for generating unique transaction IDs.

## Environment Variables

You can configure the following environment variables for the application:

- `HOST`: The hostname of the server (default: `127.0.0.1`).
- `PORT`: The port on which the application will run (default: `8080`).
- `DATA_DIR`: The directory where holiday data is stored as JSON files (default: `data`)
- `RUST_LOG`: The logging level for the application (default: `info`)

You can set these variables in your `.env` file or pass them when running the server:

```bash
HOST=localhost
PORT=8080
DATA_DIR=data
RUST_LOG=info
```

## Running the Application

To run the application, use the following command:

```bash
cargo run
```

This will start the server, and you can access it in your browser or through an API client at `http://127.0.0.1:8080/`.

### Scraping Holidays

To scrape holidays for a specific year, access:

```
GET http://127.0.0.1:8080/scrape/{year}
```

Replace `{year}` with the year you want to scrape. For example:

```
GET http://127.0.0.1:8080/scrape/2025
```

### Retrieve Holiday Data (Non-Grouped)

To get the saved holiday data for a particular year, use:

```
GET http://127.0.0.1:8080/libur/{year}
```

Example:

```
GET http://127.0.0.1:8080/libur/2025
```

### Retrieve Grouped Holiday Data

To get the holiday data for a particular year, grouped by `joint_leave` and `non_joint_leave`, use the following endpoint:

```
GET http://127.0.0.1:8080/libur/{year}/grouped
```

Example:

```
GET http://127.0.0.1:8080/libur/2025/grouped
```

## License

This project is licensed under the MIT License.
