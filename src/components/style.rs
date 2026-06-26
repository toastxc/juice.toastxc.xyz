pub struct Tree;
impl Tree {
    pub const STEM: &'static str = "font-bold text-fuchsia-300 font-mono";
    pub const LEAF: &'static str =
        "pl-5 text-fuchsia-300 hover:text-black hover:bg-fuchsia-300 font-mono";
}

pub struct Doc;
impl Doc {
    pub const TITLE: &'static str = "text-2xl font-bold font-bold text-fuchsia-300 font-mono";
    pub const HEADING: &'static str = "text-1xl font-bold font-bold text-fuchsia-300 font-mono";

    pub const LINK: &'static str =
        "hover:text-black hover:bg-fuchsia-300 underline text-fuchsia-300";
}

pub struct Table;
pub struct TableGrey;
impl Table {
    pub const TABLE: &'static str =
        "border-collapse border border-fuchsia-800  bg-fuchsia-950 w-full h-full";
    pub const LINK: &'static str = "hover:text-black hover:bg-fuchsia-300 underline";
    pub const HEAD: &'static str = "bg-fuchsia-800";
    pub const HEAD_CELL: &'static str = "text-left";
}

impl TableGrey {
    pub const TABLE: &'static str = "border-collapse border border-white   w-full h-full";
    pub const LINK: &'static str = "hover:text-black hover:bg-white-300 underline";
    pub const HEAD: &'static str = "";
    pub const HEAD_CELL: &'static str = "text-left";
}