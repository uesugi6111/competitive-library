//! グラフに関係しそうなもの
pub mod dijkstra;
pub mod dijkstra_restore_path;
pub mod euler_tour;
pub mod floyd_warshall;
pub mod heavy_light_decomposition;
pub mod lowest_common_ancestor_doubling;
pub mod lowest_common_ancestor_rmq;
pub mod minimum_spanning_tree_kruskal;
pub mod minimum_spanning_tree_prim;
pub mod shortest_path_faster_algorithm;
pub mod strongly_connected_component;
pub mod tree_diameter;
pub mod util;
pub mod zero_one_bfs;

#[deprecated(since = "0.1.0", note = "use `dijkstra_restore_path` instead")]
pub mod dijkstra_restorepath {
    pub use super::dijkstra_restore_path::*;
}

#[deprecated(since = "0.1.0", note = "use `minimum_spanning_tree_kruskal` instead")]
pub mod minimun_spanning_tree_kruskal {
    pub use super::minimum_spanning_tree_kruskal::*;
}

#[deprecated(since = "0.1.0", note = "use `minimum_spanning_tree_prim` instead")]
pub mod minimun_spanning_tree_prim {
    pub use super::minimum_spanning_tree_prim::*;
}

#[deprecated(since = "0.1.0", note = "use `tree_diameter` instead")]
pub mod tree_diamiter {
    pub use super::tree_diameter::*;
}
