use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;
use rand::Rng;
use rust_decimal::Decimal;

#[derive(Deserialize)]
struct FuelComsumptionInfo {
    distance: Decimal,
    yearOfProduction: u32,
    fuelUsagePer100KM: Decimal,
}

#[derive(Deserialize)]
struct FailProbability {
    VIN: String,
}

#[get("/calculateDisselUsageForDistance")]
async fn calculateDisselUsageForDistance(vehicle_info: web::Query<FuelComsumptionInfo>) -> Result<String>{
    let fuel_usage: Decimal = (vehicle_info.distance / Decimal::new(100,0)) * vehicle_info.fuelUsagePer100KM;
    Ok(format!("fuelUsage: {}", fuel_usage))
}

#[get("/probabilityOfUnitInjectorFail")]
async fn probabilityOfUnitInjectorFail(_vin: web::Query<FailProbability>) -> Result<String>{
    let mut rng = rand::thread_rng();
    let fail_probability = rng.gen_range(0..101);
    let res = Decimal::new(fail_probability, 2);
    Ok(format!("failProbability: {}", res.to_string().trim().replace('.',",")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new()
            .service(calculateDisselUsageForDistance)
            .service(probabilityOfUnitInjectorFail)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
