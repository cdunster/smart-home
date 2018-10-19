use rocket::response::status;
use rocket::{Rocket, State};
use rocket_contrib::Json;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Serialize)]
struct Zone {
    name: String,
}

type ZoneCollectionState = Mutex<ZoneCollection>;

#[derive(Serialize)]
pub struct ZoneCollection {
    zones: HashMap<&'static str, Zone>,
}

impl ZoneCollection {
    pub fn new() -> ZoneCollection {
        ZoneCollection {
            zones: HashMap::new(),
        }
    }

    fn add(&mut self, uuid: &'static str, zone: Zone) {
        self.zones.insert(uuid, zone);
    }

    fn get(&self, uuid: &str) -> Option<&Zone> {
        self.zones.get(uuid)
    }
}

pub fn mount(rocket: Rocket, zones: ZoneCollection) -> Rocket {
    rocket
        .mount("/zones", routes![get_zones, post_zones, get_zone_from_uuid])
        .manage(ZoneCollectionState::new(zones))
}

#[get("/", format = "application/json")]
fn get_zones(zones: State<ZoneCollectionState>) -> Json {
    Json(json!(zones.inner()))
}

#[post("/", format = "application/json")]
fn post_zones(zones: State<ZoneCollectionState>) -> status::Created<Json<Zone>> {
    let zone = Zone {
        name: "Living Room".to_string(),
    };
    zones.lock().unwrap().add("new-uuid", zone.clone());

    status::Created("/zones/new-uuid".to_string(), Some(Json(zone)))
}

#[get("/<uuid>", format = "application/json")]
fn get_zone_from_uuid(uuid: String, zones: State<ZoneCollectionState>) -> Option<Json<Zone>> {
    if let Some(zone) = zones.lock().unwrap().get(&uuid) {
        Some(Json(zone.clone()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
