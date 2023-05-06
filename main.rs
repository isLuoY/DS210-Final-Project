mod graph_builder;
mod read;
mod bfs;

use std::vec;

use read::read_file;
use read::read_label;
use bfs::compute_distance_bfs;
use crate::graph_builder::Graph;
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;

pub fn main(){
    //read data + build graph
    read_file("email-Eu-core.txt");
    let n = 1005;
    let mut edges: ListOfEdges = read_file("email-Eu-core.txt");
    edges.sort();
    let graph = Graph::create_directed(n,&edges);

    read_label("email-Eu-core-department-labels.txt");
    let n_l = 42;
    let mut edges_l: ListOfEdges = read_label("email-Eu-core-department-labels.txt");
    edges_l.sort();
    let graph_l = Graph::create_directed(n_l,&edges_l);
    //original count distribution
    let mut count = vec![[1,0],[2,0],[3,0],[4,0],[5,0],[6,0],[7,0],[8,0]];
    let mut no_connection = Vec::new();
    //bfs+count route distance num
    for k in 0..graph.n {
        compute_distance_bfs(k, &graph);
        let mut ind = 0;
        for i in compute_distance_bfs(k, &graph).iter(){
            if i == &Some(0 as u32){
                continue;
            }else if i == &Some(1 as u32){
                count[0][1] +=1;
            }else if i == &Some(2 as u32){
                count[1][1] +=1;
            }else if i == &Some(3 as u32){
                count[2][1] +=1;
            }else if i == &Some(4 as u32){
                count[3][1] +=1;
            }else if i == &Some(5 as u32){
                count[4][1] +=1;
            }else if i == &Some(6 as u32){
                count[5][1] +=1;
            }else if i == &None{
                count[6][1] +=1;
                no_connection.push([k,ind]);
            }else{
                count[7][1] +=1;
            }
            ind +=1
        }
    }
    //count no email sending department num
    let mut department_list = Vec::new();
    for num in 0..42{
        department_list.push([num+1,0]);
    }

    for route in no_connection.clone(){
        for (i, l) in graph_l.outedges.iter().enumerate() {
            for k in l{
                if k == &route[1]{
                    department_list[i][1]+=1;
                }
            }
        }
    }
    department_list.sort_by(|x, y| x[1].partial_cmp(&y[1]).unwrap());
    department_list.reverse();

    println!("{:?}",count);
    println!("{:?}",department_list);

    //test
    let sum = count[0][1]+count[1][1]+count[2][1]+count[3][1]+count[4][1]+count[5][1]+count[6][1]+count[7][1];
    let mut non_sum = 0;
    for index in 0..42{
        non_sum = non_sum + department_list[index][1]
    }
    assert_eq!(sum, 1004*1005, "The bfs function have mistakes");
    assert_eq!(non_sum, count[6][1], "The bfs function have mistakes");
}

#[test]
pub fn test(){
    
    let n: usize = 3;
    let mut edges: ListOfEdges = vec![(0,1),(1,2)];
    edges.sort();
    println!("{:?}",edges);
    let graph = Graph::create_directed(n,&edges);
    let test_bfs = compute_distance_bfs(0, &graph);
    println!("{:?}",test_bfs);
    assert_eq!(test_bfs, [Some(0),Some(1),Some(2)], "The bfs function have mistakes");
}