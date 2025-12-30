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

- `POST /api/v1/auth_tokens` - Create authentication token
- `GET /api/v1/users/current` - Get current user
- `GET /api/v1/routes.json` - List routes
- `GET /api/v1/routes/{id}.json` - Get route
- `GET /api/v1/routes/{id}/polyline.json` - Get route polyline
- `DELETE /api/v1/routes/{id}.json` - Delete route
- `GET /api/v1/trips.json` - List trips
- `GET /api/v1/trips/{id}.json` - Get trip
- `GET /api/v1/trips/{id}/polyline.json` - Get trip polyline
- `DELETE /api/v1/trips/{id}.json` - Delete trip

## License

Licensed under the Apache License, Version 2.0.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
