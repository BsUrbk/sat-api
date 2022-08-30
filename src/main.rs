use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct FuelComsumptionInfo {
    distance: u32,
    yearOfProduction: u32,
    fuelUsagePer100KM: u32,
}

#[get("/calculateDisselUsageForDistance")]
async fn calculateDisselUsageForDistance(vehicle_info: web::Query<FuelComsumptionInfo>) -> Result<String>{
    let fuel_usage = (vehicle_info.distance / 100) * vehicle_info.fuelUsagePer100KM;
    Ok(format!("fuel usage: {}", fuel_usage))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new().service(calculateDisselUsageForDistance))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
