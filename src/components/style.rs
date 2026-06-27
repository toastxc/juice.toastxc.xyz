pub struct Tree;
#[allow(dead_code)]
impl Tree {
    pub const STEM: &'static str = "font-bold text-fuchsia-300";
    pub const LEAF: &'static str = "pl-5 text-fuchsia-300 hover:text-black hover:bg-fuchsia-300";
}

pub struct Doc;
#[allow(dead_code)]
impl Doc {
    pub const TITLE: &'static str = "text-2xl font-bold font-bold text-fuchsia-300";
    pub const HEADING: &'static str = "text-1xl font-bold font-bold text-fuchsia-300";

    pub const LINK: &'static str =
        "hover:text-black hover:bg-fuchsia-300 underline text-fuchsia-300";

    pub const WARN: &'static str = "my-1 border-2 bg-red-100 p-1 text-red-900 dark:bg-red-800 dark:text-red-50 dark:shadow-white";

    pub const CAUTION: &'static str = "my-1 border-2 bg-red-100 p-1 text-red-900 dark:bg-yellow-800 dark:text-red-50 dark:shadow-white";
}

pub struct Table;

pub struct TableGrey;
#[allow(dead_code)]
impl Table {
    pub const TABLE: &'static str =
        "border-collapse border border-fuchsia-800  bg-fuchsia-950 w-full h-full";
    pub const LINK: &'static str = "hover:text-black hover:bg-fuchsia-300 underline";
    pub const HEAD: &'static str = "bg-fuchsia-800";
    pub const HEAD_CELL: &'static str = "text-left";
}
#[allow(dead_code)]
impl TableGrey {
    pub const TABLE: &'static str = "border-collapse border border-white w-full h-full";
    pub const LINK: &'static str = "hover:text-black hover:bg-white-300 underline";
    pub const HEAD: &'static str = "";
    pub const HEAD_CELL: &'static str = "text-left";
}
