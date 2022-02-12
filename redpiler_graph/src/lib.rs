use bincode::{BincodeRead, Result};
use serde::{Deserialize, Serialize};

pub type NodeId = usize;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum LinkType {
    Default,
    Side,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Link {
    pub ty: LinkType,
    pub weight: u8,
    pub to: NodeId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
    Repeater(u8),
    Comparator(ComparatorMode),
    Torch,
    StoneButton,
    StonePressurePlate,
    Lamp,
    Lever,
    Constant,
    Wire,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Node {
    pub ty: NodeType,
    pub inputs: Vec<Link>,
    pub updates: Vec<NodeId>,
    pub facing_diode: bool,
    pub comparator_far_input: Option<u8>,
    pub output_power: u8,
    /// Comparator powered / Repeater locked
    pub diode_state: bool,
    pub pos: BlockPos,
}

pub fn serialize(nodes: &[Node]) -> Result<Vec<u8>> {
    bincode::serialize(nodes)
}

pub fn serialize_into<W>(writer: W, value: &[Node]) -> Result<()>
where
    W: std::io::Write,
{
    bincode::serialize_into(writer, value)
}

pub fn deserialize(bytes: &[u8]) -> Result<Vec<Node>> {
    bincode::deserialize(bytes)
}

pub fn deserialize_from<'a, R>(reader: R) -> Result<Vec<Node>>
where
    R: BincodeRead<'a>,
{
    bincode::deserialize_from(reader)
}
