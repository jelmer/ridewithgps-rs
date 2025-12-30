# ridewithgps-client

A Rust client library for the [RideWithGPS](https://ridewithgps.com) API v1.

## Features

- Authentication with API key and auth tokens
- User management
- Route operations (list, get, get polyline, delete)
- Trip operations (list, get, get polyline, delete)
- Type-safe API with serde serialization
- Blocking HTTP client (async support planned)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ridewithgps-client = "0.1"
```

## Usage

### Basic Setup

```rust
use ridewithgps_client::RideWithGpsClient;

// Create a client with API key only
let client = RideWithGpsClient::new(
    "https://ridewithgps.com",
    "your-api-key",
    None
);

// Or authenticate with email and password
let client = RideWithGpsClient::with_credentials(
    "https://ridewithgps.com",
    "your-api-key",
    "user@example.com",
    "password"
)?;
```

### Authentication

```rust
use ridewithgps_client::RideWithGpsClient;

let mut client = RideWithGpsClient::new(
    "https://ridewithgps.com",
    "your-api-key",
    None
);

// Create an auth token
let auth = client.create_auth_token("user@example.com", "password")?;
client.set_auth_token(&auth.auth_token);
```

### Working with Routes

```rust
use ridewithgps_client::{RideWithGpsClient, ListRoutesParams, Visibility};

let client = RideWithGpsClient::new(
    "https://ridewithgps.com",
    "your-api-key",
    Some("your-auth-token")
);

// List routes with filters
let params = ListRoutesParams {
    min_distance: Some(10000.0), // 10km
    visibility: Some(Visibility::Public),
    ..Default::default()
};

let routes = client.list_routes(Some(&params))?;
for route in routes.results {
    println!("Route: {} - {:.2}km",
        route.name.unwrap_or_default(),
        route.distance.unwrap_or(0.0) / 1000.0
    );
}

// Get a specific route
let route = client.get_route(12345)?;

// Get route polyline
let polyline = client.get_route_polyline(12345)?;
println!("Polyline: {}", polyline.polyline);

// Delete a route
client.delete_route(12345)?;
```

### Working with Trips

```rust
use ridewithgps_client::{RideWithGpsClient, ListTripsParams};

let client = RideWithGpsClient::new(
    "https://ridewithgps.com",
    "your-api-key",
    Some("your-auth-token")
);

// List trips
let params = ListTripsParams {
    min_distance: Some(20000.0), // 20km
    ..Default::default()
};

let trips = client.list_trips(Some(&params))?;
for trip in trips.results {
    println!("Trip: {} - {:.2}km",
        trip.name.unwrap_or_default(),
        trip.distance.unwrap_or(0.0) / 1000.0
    );
}

// Get a specific trip
let trip = client.get_trip(67890)?;

// Get trip polyline
let polyline = client.get_trip_polyline(67890)?;

// Delete a trip
client.delete_trip(67890)?;
```

### Working with Collections

```rust
use ridewithgps_client::RideWithGpsClient;

let client = RideWithGpsClient::new(
    "https://ridewithgps.com",
    "your-api-key",
    None
);

// List all collections
let collections = client.list_collections(None)?;
for collection in collections.results {
    println!("Collection: {} - {:?}",
        collection.id,
        collection.name.unwrap_or_default()
    );
}

// Get a specific collection with all its routes and trips
let collection = client.get_collection(8094883)?;
println!("Collection: {:?}", collection.name);
println!("Description: {:?}", collection.description);

// Access routes within the collection
if let Some(routes) = &collection.routes {
    println!("\nRoutes in collection:");
    for route in routes {
        println!("  Route {} - {:?} ({:.1}km)",
            route.id,
            route.name,
            route.distance.unwrap_or(0.0) / 1000.0
        );
    }
}

// Access trips within the collection
if let Some(trips) = &collection.trips {
    println!("\nTrips in collection:");
    for trip in trips {
        println!("  Trip {} - {:?} ({:.1}km)",
            trip.id,
            trip.name,
            trip.distance.unwrap_or(0.0) / 1000.0
        );
    }
}

// Get the pinned collection (requires auth)
let client_auth = RideWithGpsClient::new(
    "https://ridewithgps.com",
    "your-api-key",
    Some("your-auth-token")
);
let pinned = client_auth.get_pinned_collection()?;
println!("Pinned collection: {:?}", pinned.name);
```

### Working with Users

```rust
use ridewithgps_client::RideWithGpsClient;

let client = RideWithGpsClient::new(
    "https://ridewithgps.com",
    "your-api-key",
    Some("your-auth-token")
);

// Get current user information
let user = client.get_current_user()?;
println!("User: {:?}", user);
```

## API Coverage

Currently implemented endpoints:

### Authentication & Users
- `POST /api/v1/auth_tokens` - Create authentication token
- `GET /api/v1/users/current` - Get current user

### Routes
- `GET /api/v1/routes.json` - List routes
- `GET /api/v1/routes/{id}.json` - Get route
- `GET /api/v1/routes/{id}/polyline.json` - Get route polyline
- `DELETE /api/v1/routes/{id}.json` - Delete route

### Trips
- `GET /api/v1/trips.json` - List trips
- `GET /api/v1/trips/{id}.json` - Get trip
- `GET /api/v1/trips/{id}/polyline.json` - Get trip polyline
- `DELETE /api/v1/trips/{id}.json` - Delete trip

### Events
- `GET /api/v1/events.json` - List events
- `POST /api/v1/events.json` - Create event
- `GET /api/v1/events/{id}.json` - Get event
- `PUT /api/v1/events/{id}.json` - Update event
- `DELETE /api/v1/events/{id}.json` - Delete event

### Collections
- `GET /api/v1/collections.json` - List collections
- `GET /api/v1/collections/{id}.json` - Get collection
- `GET /api/v1/collections/pinned.json` - Get pinned collection

### Sync
- `GET /api/v1/sync.json` - Get changed items since datetime

### Points of Interest (organization-only)
- `GET /api/v1/points_of_interest.json` - List POIs
- `POST /api/v1/points_of_interest.json` - Create POI
- `GET /api/v1/points_of_interest/{id}.json` - Get POI
- `PUT /api/v1/points_of_interest/{id}.json` - Update POI
- `DELETE /api/v1/points_of_interest/{id}.json` - Delete POI
- `POST /api/v1/points_of_interest/{id}/routes/{route_id}.json` - Associate POI with route
- `DELETE /api/v1/points_of_interest/{id}/routes/{route_id}.json` - Disassociate POI from route

### Club Members (organization-only)
- `GET /api/v1/members.json` - List members
- `GET /api/v1/members/{id}.json` - Get member
- `PUT /api/v1/members/{id}.json` - Update member permissions/status

**All RideWithGPS API v1 endpoints are now implemented!**

## License

Licensed under the Apache License, Version 2.0.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
