#[derive(Debug)]
struct Node {
    nodeId: usize,
    nodeName: String,
}


#[derive(Debug, Clone)]
struct Edge {
    edge: bool
}

#[derive(Debug)]
struct Graph {
    nodeNums: usize,
    edges: Vec<Vec<Edge>>,
}


impl Node {
    fn new(nodeId: usize, nodeName: String) -> Node {
        Node{
            nodeId:nodeId,
            nodeName:nodeName
        }
    }
}

impl Edge{
    fn new()->Edge{
        Edge{
            edge:false
        }
    }

    fn have_edge()->Edge{
        Edge{
            edge:true
        }
    }
}

impl Graph{
    fn new(nums:usize)->Graph{
        Graph{
            nodeNums:nums,
            edges:vec![vec![Edge::new();nums];nums]
        }
    }

    fn insert_edge(&mut self,v1:Node,v2:Node){
        match v1.nodeId<self.nodeNums && v2.nodeId<self.nodeNums {
            true=>{
                self.edges[v1.nodeId][v2.nodeId]=Edge::have_edge();
            }
            false=>{
                panic!("your node id is bigger than node nums!");
            }
        }
    }


}

pub fn test_simply_graph() {

    println!("{}","-----start simply graph test");

    let mut g = Graph::new(2);
    let v1 = Node::new(0, "v1".to_string());
    let v2 = Node::new(1, "v2".to_string());
    g.insert_edge(v1,v2);
    println!("{:?}", g);

    println!("{}","-----end simply graph test");
}

