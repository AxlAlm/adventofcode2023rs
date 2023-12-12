// fn parse_graph(input: &str) -> (Graph, Vec<Node>) {
//     let mut size_x = 0;
//     let mut size_y = 0;
//     let mut galaxies = vec![];
//     for (y, line) in input.lines().enumerate() {
//         let ns: Vec<&str> = line.trim().split("").filter(|x| x != &"").collect();
//         size_x = ns.len();
//         size_y = y + 1;
//         for (x, n) in ns.into_iter().enumerate() {
//             if n == "#" {
//                 galaxies.push(Node {
//                     x: x as i32,
//                     y: y as i32,
//                 })
//             }
//         }
//     }

//     return (
//         Graph {
//             size_x: size_x as i32,
//             size_y: size_y as i32,
//         },
//         galaxies,
//     );
// }

// #[derive(Debug, Clone)]
// struct Graph {
//     size_x: i32,
//     size_y: i32,
// }

// impl Graph {
//     fn get_neighbors(&self, node: &Node) -> Vec<Node> {
//         let mut neighbors = vec![];
//         let offsets = [(0, 1), (0, -1), (-1, 0), (1, 0)];

//         for (x, y) in offsets.iter() {
//             let new_x = node.x + x;
//             let new_y = node.y + y;

//             if new_x < 0 || new_y < 0 || new_x > self.size_x || new_y > self.size_y {
//                 continue;
//             }
//             neighbors.push(Node { x: new_x, y: new_y })
//         }

//         return neighbors;
//     }
// }

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// struct Node {
//     x: i32,
//     y: i32,
// }


//  1  function Dijkstra(Graph, source):
//  2
//  3      for each vertex v in Graph.Vertices:
//  4          dist[v] ← INFINITY
//  5          prev[v] ← UNDEFINED
//  6          add v to Q
//  7      dist[source] ← 0
//  8
//  9      while Q is not empty:
// 10          u ← vertex in Q with min dist[u]
//


//1  S ← empty sequence
// 2  u ← target
// 3  if prev[u] is defined or u = source:          // Do something only if the vertex is reachable
// 4      while u is defined:                       // Construct the shortest path with a stack S
// 5          insert u at the beginning of S        // Push the vertex onto the stack
// 6          u ← prev[u]                           // Traverse from target to source

// fn shortest_path(graph: &Graph, start_node: Node, target_node: Node) -> usize {
//     let mut pq = PriorityQueue::new();
//     pq.push(start_node.clone(), 99999999);
//     let mut costs: HashMap<Node, i32> = HashMap::new();
//     let mut came_from: HashMap<Node, Node> = HashMap::new();

//     while !pq.is_empty() {
//         let (current_node, _) = pq.pop().unwrap();
//         // dbg!(&current_node);

//         if current_node == target_node {
//             break;
//         }

//         let neighbors: Vec<Node> = graph.get_neighbors(&current_node);
//         dbg!(&current_node, &neighbors);
//         for neighbor in neighbors.iter() {
//             let current_neighbor_cost = costs.get(neighbor).unwrap_or(&0).clone();
//             let new_cost = costs.get(&current_node).unwrap_or(&0).clone() + 1;
//             // + graph.cost(&current_node, neighbor);
//             // + graph.cost(neighbor, &target_node);

//             dbg!(&new_cost);

//             // dbg!(
//             //     !costs.contains_key(neighbor),
//             //     new_cost > current_neighbor_cost
//             // );
//             // dbg!((!costs.contains_key(neighbor) || new_cost < current_neighbor_cost));
//             if !costs.contains_key(neighbor) || new_cost < current_neighbor_cost {
//                 costs.insert(neighbor.clone(), new_cost);
//                 pq.push(
//                     neighbor.clone(),
//                     new_cost + manhattan_distance(neighbor, &target_node),
//                 );
//                 came_from.insert(neighbor.clone(), current_node.clone());
//             }
//         }
//     }

//     let mut next = target_node.clone();
//     let mut path = vec![];
//     while next != start_node.clone() {
//         path.push(next.clone());
//         next = came_from.get(&next).unwrap().clone();
//     }

//     return path.len();
// }

