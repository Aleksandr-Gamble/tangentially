//! this module contains structs and logic for making graphs as displayed in the background of
//! xtchd.com 

use std::{fmt, collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_json;


/// 3d-force-directed is a javascript library. As such, it can accept a wide range of objects as a node: the key properties are that each
/// node has a .id and .name property.  
/// In contrast, rust is a strictly typed language. 
/// The Node struct, and the associated ToNode trait, try to balance that by using generics to ensure the key properties needed for the 3d-force-directed libray
/// are in place while alowing enough flexability to accept properties that will be unique to a given "NodeVariant". It does this by using three generics:
/// NV: short for NodeVariant: typically an enum of possible "node types" upon which std::fmt::Display is implemented. Or you could be lazy and use just String.  
/// PK: the primary key for the selected variant: this would typically be String or i32 or a tuple (i32, i16, String) etc.  
/// T: A generic struct for capturing other properties specific to this node type 
#[derive(Serialize, Deserialize, Clone)]
pub struct Node<NV, PK, T> {
    /// This will indicate the "node type": typically it is a vanriant of the NV enum, although a simple String would work fine 
    pub variant: NV,
    /// The primary key within the variant type, typically i32, String, or a tuple
    pub variant_pk: PK,
    /// This is the id used by 3d-force-graph to identify a unique node
    pub id: String,
    /// This is the name as displayed in the graph for a node 
    pub name: String,
    /// The props field captures any props specific to the selected variant 
    pub props: T,
}

impl<NV: Serialize, PK: Serialize, T: Serialize> Node<NV, PK, T>  {
    pub fn to_node_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(&self)
    }
}



/// Implementing this trait on a struct will makes it ergonomic to convert it to a node 
pub trait ToNode<NV: fmt::Display, PK: fmt::Debug, T> {
    fn node_variant(&self) -> NV;
    fn node_pk(&self) -> PK;
    fn node_id(&self) -> String {
        format!("{}|{:?}", &self.node_variant(), &self.node_pk())
    }
    fn node_name(&self) -> String;
    fn node_image_url(&self) -> Option<String> {
        None
    }
    fn node_props(&self) -> T;
    fn to_node(&self) -> Node<NV, PK, T> {
        let variant = self.node_variant();
        let variant_pk = self.node_pk();
        let id = self.node_id();
        let name = self.node_name();
        let props = self.node_props();
        Node{variant, variant_pk, id, name, props}
    }
    /// Edes can have labels too in 3d-force=directed. This optional method captures the "nodes' contribution" to the endge label
    /// if it is an edge source 
    fn edge_source_comment(&self) -> Option<String> {
        None
    }
    /// Edes can have labels too in 3d-force=directed. This optional method captures the "nodes' contribution" to the endge label
    /// if it is an edge target 
    fn edge_target_comment(&self) -> Option<String> {
        None
    }
}


/// If a struct already implements ToNode, ToNodeJSON makes it easy to turn it into JSON! 
pub trait ToNodeJSON<NV, PK, T>: ToNode<NV, PK, T> where 
    NV: Serialize + fmt::Display,
    PK: Serialize + fmt::Debug, 
    T:  Serialize  
{
    fn to_node_and_json(&self) -> Result<(Node<NV, PK, T>, serde_json::Value), serde_json::Error> {
        let node = self.to_node();
        let json = node.to_node_json()?;
        Ok((node, json))
    }

    fn to_node_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        let (_node, json) = self.to_node_and_json()?;
        Ok(json)
    }
}





/// The edge struct represents an edge between two nodes 
/// the <EV> property captures the type of edge: it typically will be a String or an enum that implements std::fmt::Display 
#[derive(Serialize, Deserialize)]
pub struct Edge<EV, PK, T> {
    /// This variant will 
    pub variant: EV,   
    /// The primary key within the variant type, typically i32, String, or a tuple
    pub variant_pk: PK,
    /// This id will be unique to the edge, even if other edges share the same source and destination
    pub id: String,
    /// the string corresponding to the source node id
    pub source: String,
    /// the string corresponding to the target node id
    pub target: String,
    /// An arbitrary struct to capture properties for this node 
    pub props: T,
}


impl<EV: Serialize, PK: Serialize, T: Serialize> Edge<EV, PK, T>  {
    pub fn to_edge_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(&self)
    }
}



/// Implementing this trait on a struct will makes it ergonomic to convert it to an edge 
pub trait ToEdge<EV: fmt::Display, PK: fmt::Debug, T> {
    fn edge_variant(&self) -> EV;
    fn edge_pk(&self) -> PK;
    fn edge_id(&self) -> String {
        format!("{}|{:?}", &self.edge_variant(), &self.edge_pk())
    }
    fn edge_source(&self) -> String;
    fn edge_target(&self) -> String;
    fn edge_props(&self) -> T;
    fn to_edge(&self) -> Edge<EV, PK, T> {
        let variant = self.edge_variant();
        let variant_pk = self.edge_pk();
        let id = self.edge_id();
        let source = self.edge_source();
        let target = self.edge_target();
        let props = self.edge_props();
        Edge{variant, variant_pk, id, source, target, props}
    }
}


/// If a struct already implements ToEdge, ToEdgeJSON makes it easy to turn it into JSON! 
pub trait ToEdgeJSON<EV, PK, T>: ToEdge<EV, PK, T> where 
    EV: Serialize + fmt::Display,
    PK: Serialize + fmt::Debug, 
    T:  Serialize  
{
    fn to_edge_and_json(&self) -> Result<(Edge<EV, PK, T>, serde_json::Value), serde_json::Error> {
        let edge = self.to_edge();
        let json = edge.to_edge_json()?;
        Ok((edge, json))
    }
    fn to_edge_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        let (_edge, json) = self.to_edge_and_json()?;
        Ok(json)
    }
}


/// A graph contains both nodes and edges, collected first by type and next by id 
/// However, nodes and edges are reduced to simply serde_json::Value objects!  
/// This is because this struct is intended for serialization for http transmission
/// and use by a browser using JavaScript 
#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, HashMap<String, serde_json::Value>>,
    pub edges: HashMap<String, HashMap<String, serde_json::Value>>,

}


impl Graph
{
    /// return a new empty graph 
    pub fn new() -> Self {
        let nodes = HashMap::new();
        let edges = HashMap::new();
        Graph{nodes, edges}
    }


    pub fn add_node<NV, PK, T>(&mut self, node: &Node<NV, PK, T>) -> Result<(), serde_json::Error> where 
        NV: Serialize + fmt::Display,
        PK: Serialize + fmt::Debug, 
        T:  Serialize  
    {
        let json = node.to_node_json()?;
        let collection = node.variant.to_string();
        let id = node.id.clone();
        let _x = self.nodes
            .entry(collection)
            .or_insert(HashMap::new())
            .insert(id, json);
        Ok(())
    }


    pub fn add_node_from<NV, PK, T>(&mut self, n : &(dyn ToNodeJSON<NV, PK, T>)) -> Result<Node<NV, PK, T>, serde_json::Error> where 
        NV: Serialize + fmt::Display,
        PK: Serialize + fmt::Debug, 
        T:  Serialize  
    {
        let node = n.to_node();
        self.add_node(&node)?;
        Ok(node)

    }


    // by making this method private, the user must use source_edge_target() etc., ensuring the nodes that go with the edge are populated 
    fn add_edge<EV, PK, T>(&mut self, edge: &Edge<EV, PK, T>) -> Result<(), serde_json::Error> where 
        EV: Serialize + fmt::Display,
        PK: Serialize + fmt::Debug, 
        T:  Serialize  
    {
        let json = edge.to_edge_json()?;
        let collection = edge.variant.to_string();
        let id = edge.id.clone();
        let _x = self.edges
            .entry(collection)
            .or_insert(HashMap::new())
            .insert(id, json);
        Ok(())
    }

    /*// by making this method private, the user must use source_edge_target() etc., ensuring the nodes that go with the edge are populated 
    fn add_edge_from<EV, PK, T>(&mut self, e : &(dyn ToEdgeJSON<EV, PK, T>)) -> Result<Edge<EV, PK, T>, serde_json::Error> where 
        EV: Serialize + fmt::Display,
        PK: Serialize + fmt::Debug, 
        T:  Serialize  
    {
        let edge = e.to_edge();
        self.add_edge(&edge)?;
        Ok(edge)
    }*/

    
    pub fn source_edge_target<NVS, PKS, TS, EV, ET, NVT, PKT, TT>(&mut self, n_source: &(dyn ToNode<NVS, PKS, TS>), n_target: &(dyn ToNode<NVT, PKT, TT>), edge_variant: EV, edge_props: ET)
        -> Result<(Node<NVS, PKS, TS>, Edge<EV, (PKS, PKT), ET>, Node<NVT, PKT, TT>), serde_json::Error> 
    where 
        NVS: Serialize + fmt::Display,      // Node Variant, Source
        PKS: Serialize + fmt::Debug,        // Primary Key, Source
        TS:  Serialize,                     // property Type, Source
        EV: Serialize + fmt::Display,       // Edge Variant
        ET:  Serialize,                     // Edge property Type
        NVT: Serialize + fmt::Display,      // Node Variant, Target
        PKT: Serialize + fmt::Debug,        // Primary Key, Target
        TT:  Serialize,                     // property Type, Target 
    {
        let source = n_source.to_node();
        let target = n_target.to_node();
        let id = format!("{:?}|{}|{:?}", &n_source.node_pk(), &edge_variant, &n_target.node_pk());
        let edge: Edge<EV, (PKS, PKT), ET>  = Edge{
            variant: edge_variant,
            variant_pk: (n_source.node_pk(), n_target.node_pk()),
            id, 
            source: n_source.node_id(),
            target: n_target.node_id(),
            props: edge_props,
        };
        self.add_node(&source)?;
        self.add_edge(&edge)?;
        self.add_node(&target)?;
        Ok((source, edge, target))
    }
}




/// this trait simply means you define an implementation of to_graph, which must return a Graph struct 
pub trait ToGraph {
    /// Define how an existing graph should have content added to it from this struct 
    fn mut_graph(&self, graph: &mut Graph) -> Result<(), serde_json::Error>;
    fn to_graph(&self) -> Result<Graph, serde_json::Error> {
        let mut g = Graph::new();
        self.mut_graph(&mut g)?;
        Ok(g)
    }
}


/// If a struct implements ToGraph, you may wish for it to also implement FocusNode.  
/// The idea here is that if you display a (sub)graph for the user to see,
/// you may wish to zoom to a particular node as the 'starting point'
/// or other node of interest. 
/// The ZoomNode trait allows that functionality by providing a variant and variant_pk,
/// much like the ToNode trait 
pub trait ZoomNode<EV: fmt::Display, PK: fmt::Debug>: ToGraph {
    /// EV is the variant, PK is the variant_pk
    fn zoom_to(&self) -> Option<(EV, PK)>;
    /// the graph3d.js :: zoomToId() function expects a node id = 'variant|PK'
    fn zoom_to_id(&self) -> Option<String> {
        match self.zoom_to() {
            Some((variant, pk)) => Some(format!("{}|{:?}", variant, pk)),
            None => None,
        }
    }
}