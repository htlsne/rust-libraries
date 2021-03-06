use crate::graph_struct::{Edge, Graph};

// 幅優先探索
pub fn bfs<E: Edge>(start: usize, graph: &Graph<E>) {
    use std::collections::VecDeque;

    let n = graph.len();

    let mut visited = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        // replace it
        eprintln!("visit {}", vertex);
        for edge in graph.edges(vertex) {
            let next = edge.to();
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }
}

// closureを受け取る幅優先探索
pub fn bfs_closure<F>(start: usize, graph_list: &[Vec<usize>], mut func: F)
where
    F: FnMut(usize, usize),
{
    use std::collections::VecDeque;

    let n = graph_list.len();

    let mut visited = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(vertex) = queue.pop_front() {
        for &next in &graph_list[vertex] {
            if !visited[next] {
                func(vertex, next);
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }
}

pub fn bfs_distance<E: Edge>(start: usize, graph: &Graph<E>) -> Vec<u64> {
    use std::collections::VecDeque;

    let n = graph.len();

    let mut visited = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    const INF: u64 = std::u64::MAX >> 2;
    let mut distances = vec![INF; graph.len()];
    distances[start] = 0;

    while let Some(vertex) = queue.pop_front() {
        for edge in graph.edges(vertex) {
            let next = edge.to();
            if !visited[next] {
                visited[next] = true;
                distances[next] = distances[vertex] + 1;
                queue.push_back(next);
            }
        }
    }

    distances
}

// 深さ優先探索
pub fn dfs<E: Edge>(start: usize, graph: &Graph<E>) {
    let n = graph.len();

    let mut visited = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();

    visited[start] = true;
    stack.push(start);

    while let Some(vertex) = stack.pop() {
        eprintln!("visit {}", vertex);
        for edge in graph.edges(vertex) {
            let next = edge.to();
            if !visited[next] {
                visited[next] = true;
                stack.push(next);
            }
        }
    }
}

// 深さ優先探索（再帰）
pub fn dfs_recursive<E: Edge>(start: usize, graph: &Graph<E>) {
    let n = graph.len();
    let mut visited = vec![false; n];

    dfs_aux(start, graph, &mut visited);
}
// 補助関数
pub fn dfs_aux<E: Edge>(start: usize, graph: &Graph<E>, visited: &mut Vec<bool>) {
    visited[start] = true;

    eprintln!("visit {}", start);

    for edge in graph.edges(start) {
        let next = edge.to();
        if !visited[next] {
            dfs_aux(next, &graph, visited);
        }
    }
}

// クロージャを受け取るdfs
pub fn dfs_closure<F>(current: usize, graph_list: &[Vec<usize>], mut func: F)
where
    F: FnMut(usize, usize),
{
    let n = graph_list.len();
    let mut visited = vec![false; n];

    dfs_closure_aux(current, graph_list, &mut visited, &mut func);
}
// 補助関数
pub fn dfs_closure_aux<F>(
    start: usize,
    graph_list: &[Vec<usize>],
    visited: &mut Vec<bool>,
    func: &mut F,
) where
    F: FnMut(usize, usize),
{
    visited[start] = true;

    for &next in &graph_list[start] {
        if !visited[next] {
            func(start, next);
            dfs_closure_aux(next, &graph_list, visited, func);
        }
    }
}

#[derive(Clone)]
pub struct EdgeWithLength {
    to: usize,
    len: u64,
}

impl EdgeWithLength {
    pub fn new(to: usize, len: u64) -> EdgeWithLength {
        EdgeWithLength { to, len }
    }
}

impl Edge for EdgeWithLength {
    fn to(&self) -> usize {
        self.to
    }
}

// ダイクストラ法
pub fn dijkstra(start: usize, graph: &Graph<EdgeWithLength>) -> Vec<u64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = graph.len();
    let mut distances = vec![std::u64::MAX >> 2; n];
    distances[start] = 0;

    // BinaryHeapは最大ヒープなので、Reverseで距離最小のものを取り出せるようにする
    // ヒープの中身は Reverse((distance, distination))
    let mut queue: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
    queue.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = queue.pop() {
        if distances[u] < d {
            continue;
        }
        for edge in graph.edges(u) {
            let adj = edge.to;
            let edge_len = edge.len;
            let alt = d + edge_len;
            if distances[adj] > alt {
                distances[adj] = alt;
                queue.push(Reverse((alt, adj)))
            }
        }
    }

    distances
}

// Warshall-Floyd法(長さ1)
#[allow(unused)]
pub fn warshall_floyd(graph_mat: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = graph_mat.len();

    let mut d = vec![vec![std::usize::MAX >> 2; n]; n];
    for i in 0..n {
        for j in 0..n {
            if graph_mat[i][j] == 1 {
                d[i][j] = 1;
            }
        }
    }
    for i in 0..n {
        d[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }

    d
}

// グラフの直径
#[allow(unused)]
pub fn graph_diameter(graph_mat: &[Vec<usize>]) -> usize {
    let d_mat = warshall_floyd(&graph_mat);
    let mut diameter = 0;
    for v in d_mat {
        for d in v {
            if d > diameter {
                diameter = d;
            }
        }
    }

    diameter
}

// 木の直径．グラフが閉路を持たないときのみ使える。
#[allow(unused)]
pub fn tree_diameter(graph: &Graph<EdgeWithLength>) -> u64 {
    let start = 0;
    let d_v = dijkstra(start, graph);

    let mut farthest = start;
    let mut d_max = 0;
    for (v, &d) in d_v.iter().enumerate() {
        if d > d_max {
            d_max = d;
            farthest = v;
        }
    }

    let start = farthest;
    let mut d_max = 0;
    let d_v = dijkstra(start, graph);
    for (_, &d) in d_v.iter().enumerate() {
        if d > d_max {
            d_max = d;
        }
    }
    d_max
}

// 2部グラフ判定
#[allow(unused)]
pub fn is_bipartite_graph(graph_list: &[Vec<usize>]) -> bool {
    let n = graph_list.len();

    let mut stack: Vec<(usize, bool)> = vec![];
    let mut colors = vec![None; n];

    let start = 0;
    stack.push((start, true));

    while let Some((vertex, color)) = stack.pop() {
        colors[vertex] = Some(color);

        for &next in &graph_list[vertex] {
            match colors[next] {
                Some(next_color) => {
                    if color == next_color {
                        return false;
                    }
                }
                None => stack.push((next, !color)),
            }
        }
    }

    true
}

pub fn topological_sort(graph_list: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = graph_list.len();

    let mut incoming_num = vec![0; n];
    for edges in graph_list.iter() {
        for &vertex in edges {
            incoming_num[vertex] += 1;
        }
    }

    // 入次数0の頂点集合
    let mut stack: Vec<usize> = vec![];
    for (vertex, &num) in incoming_num.iter().enumerate() {
        if num == 0 {
            stack.push(vertex);
        }
    }

    let mut ret = vec![];
    while let Some(vertex) = stack.pop() {
        ret.push(vertex);
        for &next in &graph_list[vertex] {
            incoming_num[next] -= 1;
            if incoming_num[next] == 0 {
                stack.push(next);
            }
        }
    }

    if incoming_num.iter().filter(|a| **a != 0).count() == 0 {
        Some(ret)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod dijksta {
        use super::*;

        #[test]
        fn graph1() {
            // graph shape:
            // 0 - 1 - 3
            //   \
            //     2
            let n = 4;
            let graph_mat = vec![
                vec![0, 1, 1, 0],
                vec![1, 0, 0, 1],
                vec![1, 0, 0, 0],
                vec![0, 1, 0, 0],
            ];
            let mut graph_with_length = Graph::new(n);
            for i in 0..n {
                for j in 0..n {
                    if graph_mat[i][j] == 1 {
                        graph_with_length.add_edge(i, EdgeWithLength::new(j, 1));
                    }
                }
            }
            let ds = dijkstra(0, &graph_with_length);
            assert_eq!(vec![0, 1, 1, 2], ds);
        }
    }
}
