use flucoma_rs::search::KDTree;

fn main() {
    // 1. Create a new KDTree with 2 dimensions
    let mut tree = KDTree::new(2);

    // 2. Add some named points to the tree
    // Each point must match the tree's dimensionality (2 in this case)
    tree.add("origin", &[0.0, 0.0]);
    tree.add("right",  &[10.0, 0.0]);
    tree.add("up",     &[0.0, 10.0]);
    tree.add("diagonal", &[7.0, 7.0]);

    println!("KDTree created and populated with 4 points.");

    // 3. Query the tree for the 2 nearest neighbors to a target point
    let target = [1.0, 1.0];
    let k = 2;
    let result = tree.k_nearest(&target, k);

    println!("\nSearching for {} nearest neighbors to {:?}", k, target);
    
    // 4. Print results
    for i in 0..result.ids.len() {
        println!(
            "  {}. ID: {:<10} | Distance: {:.4}",
            i + 1,
            result.ids[i],
            result.distances[i]
        );
    }

    // 5. Another query
    let target2 = [8.0, 2.0];
    let result2 = tree.k_nearest(&target2, 1);
    
    println!("\nSearching for 1 nearest neighbor to {:?}", target2);
    if let Some(id) = result2.ids.first() {
        println!("  Nearest is: {} (distance: {:.4})", id, result2.distances[0]);
    }
}
