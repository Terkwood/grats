#[derive(Debug)]
pub struct History {
    pub days: Vec<Day>,
    pub offset: FixedOffset,
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
pub struct Day {
    pub date: Date<FixedOffset>,
    pub inventory: Inventory,
}

impl History {
    pub fn days(&self) {
        todo!()
    }
}
