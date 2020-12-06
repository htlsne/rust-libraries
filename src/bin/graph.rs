use competitive_rust_libraries::graph::*;

fn main() {
    let n = 4;
    // graph shape:
    // 0 - 1 - 3
    //   \
    //     2
    let graph_mat = vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 0, 1],
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
    ];
    eprintln!("{:?}", graph_mat);

    let mut graph_list = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if graph_mat[i][j] == 1 {
                graph_list[i].push(j);
            }
        }
    }
    eprintln!("{:?}", graph_list);

    println!("start bfs");
    bfs(0, &graph_list);
    println!();

    println!("start dfs");
    dfs(0, &graph_list);
    println!();

    println!("start dfs recursive");
    dfs_recursive(0, &graph_list);

    println!("start dijkstra");
    let mut graph_with_distance = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if graph_mat[i][j] == 1 {
                graph_with_distance[i].push((j, 1));
            }
        }
    }
    let distances = dijkstra(0, &graph_with_distance);
    println!("{:?}", distances);

    // graph shape:
    // 0 -> 1 -> 3
    //   ↘
    //     2
    let mut graph_list2 = vec![vec![]; n];
    graph_list2[0].push(1);
    graph_list2[0].push(2);
    graph_list2[1].push(3);
    println!("Topological Sort");
    match topological_sort(&graph_list2) {
        Some(v) => println!("{:?}", v),
        None => println!("Graph has a closed path.")
    }

    // graph shape:
    // 0 -> 1
    //   ↖  ↓
    //      2
    let mut graph_list3 = vec![vec![]; n];
    graph_list3[0].push(1);
    graph_list3[1].push(2);
    graph_list3[2].push(0);
    println!("Topological Sort");
    match topological_sort(&graph_list3) {
        Some(v) => println!("{:?}", v),
        None => println!("Graph has a closed path.")
    }
}