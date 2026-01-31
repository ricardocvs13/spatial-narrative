//! Clustering Example
//!
//! Demonstrates DBSCAN and KMeans spatial clustering algorithms
//! for grouping geographic locations.
//!
//! Run with: `cargo run --example clustering`

use spatial_narrative::analysis::{ClusteringResult, DBSCAN, KMeans};
use spatial_narrative::core::{Event, Location, Timestamp};

fn main() {
    println!("=== Spatial Clustering Example ===\n");

    // Create events representing different activity clusters
    // Cluster 1: Coffee shops in Midtown Manhattan
    let midtown_events = vec![
        ("Starbucks 42nd St", 40.7549, -73.9840),
        ("Blue Bottle Coffee", 40.7560, -73.9830),
        ("Joe Coffee", 40.7545, -73.9850),
        ("Gregorys Coffee", 40.7555, -73.9845),
        ("Dunkin", 40.7548, -73.9835),
    ];

    // Cluster 2: Restaurants in SoHo
    let soho_events = vec![
        ("Balthazar", 40.7228, -73.9984),
        ("The Dutch", 40.7235, -73.9990),
        ("Raoul's", 40.7230, -73.9978),
        ("Lure Fishbar", 40.7225, -73.9988),
    ];

    // Cluster 3: Museums on Museum Mile
    let museum_events = vec![
        ("Metropolitan Museum", 40.7794, -73.9632),
        ("Guggenheim", 40.7830, -73.9590),
        ("Neue Galerie", 40.7812, -73.9603),
        ("Jewish Museum", 40.7850, -73.9570),
    ];

    // Noise point (isolated)
    let noise_events = vec![("Random spot in Brooklyn", 40.6892, -73.9857)];

    // Combine all events
    let mut events = Vec::new();
    let mut timestamp = 0i64;

    for (name, lat, lon) in midtown_events
        .iter()
        .chain(soho_events.iter())
        .chain(museum_events.iter())
        .chain(noise_events.iter())
    {
        timestamp += 3600; // 1 hour apart
        events.push(Event::new(
            Location::new(*lat, *lon),
            Timestamp::from_unix_millis(1705741200000 + timestamp * 1000).unwrap(),
            *name,
        ));
    }

    let separator = "-".repeat(40);

    println!("Total locations: {}", events.len());
    println!();

    // === DBSCAN Clustering ===
    println!("🔵 DBSCAN Clustering");
    println!("{}", separator);
    println!("DBSCAN finds clusters of arbitrary shape based on density.");
    println!("Parameters: eps (radius in meters), min_points");
    println!();

    // Create DBSCAN clusterer
    // eps = 1000m (1km), min_points = 3
    let dbscan = DBSCAN::new(1000.0, 3);
    let result = dbscan.cluster(&events);

    print_clustering_result(&result, "DBSCAN");

    // Print cluster assignments with event names
    println!("\nCluster Assignments:");
    for (i, event) in events.iter().enumerate() {
        let cluster_id = result.labels.get(i).copied().unwrap_or(-1);
        let cluster_label = if cluster_id < 0 {
            "NOISE".to_string()
        } else {
            format!("Cluster {}", cluster_id)
        };
        println!("  {} → {}", event.text, cluster_label);
    }

    // === KMeans Clustering ===
    println!("\n🔴 KMeans Clustering");
    println!("{}", separator);
    println!("KMeans partitions data into k clusters with nearest mean.");
    println!("Parameters: k (number of clusters), max_iterations");
    println!();

    // Create KMeans clusterer with k=3 (we expect 3 clusters)
    let kmeans = KMeans::new(3);
    let result = kmeans.cluster(&events);

    print_clustering_result(&result, "KMeans");

    // Show cluster centroids
    println!("\nCluster Centroids:");
    for (i, cluster) in result.clusters.iter().enumerate() {
        let centroid = &cluster.centroid;
        println!(
            "  Cluster {}: ({:.4}, {:.4}) - {} members",
            i,
            centroid.lat,
            centroid.lon,
            cluster.event_indices.len()
        );
    }

    // === Comparing Different Parameters ===
    println!("\n⚙️ Parameter Sensitivity");
    println!("{}", separator);
    println!("Trying different DBSCAN parameters:\n");

    let params = [(500.0, 2), (1000.0, 2), (1500.0, 2), (1000.0, 4)];

    for (eps, min_pts) in params.iter() {
        let dbscan = DBSCAN::new(*eps, *min_pts);
        let result = dbscan.cluster(&events);
        let noise_count = result.labels.iter().filter(|&&l| l < 0).count();
        println!(
            "  eps={:.0}m, min_pts={}: {} clusters, {} noise points",
            eps,
            min_pts,
            result.clusters.len(),
            noise_count
        );
    }

    // === Working with Clusters ===
    println!("\n📊 Cluster Analysis");
    println!("{}", separator);

    let dbscan = DBSCAN::new(1000.0, 3);
    let result = dbscan.cluster(&events);

    for (i, cluster) in result.clusters.iter().enumerate() {
        println!("\nCluster {} Statistics:", i);
        println!("  Members: {}", cluster.event_indices.len());

        // Get member events
        let member_events: Vec<&Event> = cluster
            .event_indices
            .iter()
            .map(|&idx| &events[idx])
            .collect();

        // Print member names
        println!("  Locations:");
        for event in &member_events {
            println!("    - {}", event.text);
        }

        // Calculate cluster radius (max distance from centroid)
        let centroid = &cluster.centroid;
        let max_dist = member_events
            .iter()
            .map(|e| haversine(&e.location, centroid))
            .fold(0.0f64, f64::max);
        println!("  Radius: {:.0} meters", max_dist);
    }

    println!("\n✅ Clustering analysis complete!");
}

fn print_clustering_result(result: &ClusteringResult, name: &str) {
    println!("{} Results:", name);
    println!("  Clusters found: {}", result.clusters.len());

    let noise_count = result.labels.iter().filter(|&&l| l < 0).count();
    println!("  Noise points: {}", noise_count);

    for (i, cluster) in result.clusters.iter().enumerate() {
        println!("  Cluster {}: {} members", i, cluster.event_indices.len());
    }
}

/// Simple haversine distance calculation
fn haversine(loc1: &Location, loc2: &Location) -> f64 {
    let r = 6_371_000.0; // Earth's radius in meters

    let lat1 = loc1.lat.to_radians();
    let lat2 = loc2.lat.to_radians();
    let dlat = (loc2.lat - loc1.lat).to_radians();
    let dlon = (loc2.lon - loc1.lon).to_radians();

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    r * c
}
