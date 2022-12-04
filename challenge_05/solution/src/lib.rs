use std::fmt;

pub struct Room {
    pub name: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = &self.clone().name;
        let north = &self.clone().north;
        let south = &self.clone().south;
        let west = &self.clone().west;
        let _east = &self.clone().east;

        let mut box_size = 0;
        let gap = " ".repeat(3 + west.clone().len());

        if north.clone().len() >= name.clone().len() && north.clone().len() >= south.clone().len() {
            box_size = 4 + north.clone().len();
        }
        if name.clone().len() >= north.clone().len() && name.clone().len() >= south.clone().len() {
            box_size = 4 + name.clone().len();
        }
        if south.clone().len() >= name.clone().len() && south.clone().len() >= north.clone().len() {
            box_size = 4 + south.clone().len();
        }
        box_size = box_size + 1 - box_size % 2;

        let line_1_padding = box_size.clone() - north.clone().len() - 4;
        let line_3_padding = (box_size.clone() - 4) / 2;
        let line_4_padding = box_size.clone() - name.clone().len() - 4;
        let line_7_padding = box_size.clone() - south.clone().len() - 4;

        let line_1 = gap.clone() + "[ " + &" ".repeat(line_1_padding.clone() / 2) + north + &" ".repeat(line_1_padding.clone() / 2 + line_1_padding.clone() % 2) + " ]";
        let line_2 = gap.clone() + &" ".repeat(box_size.clone() / 2) + "|";
        let line_3 = gap.clone() + "+-" + &"-".repeat(line_3_padding) + "N" + &"-".repeat(line_3_padding) + "-+";
        let line_4 = west.clone() + " - | " + &" ".repeat(line_4_padding.clone() / 2) + name + &" ".repeat(line_4_padding.clone() / 2 + line_4_padding.clone() % 2) + " | - " + _east;
        let line_5 = gap.clone() + "+-" + &"-".repeat(line_3_padding) + "S" + &"-".repeat(line_3_padding) + "-+";
        let line_6 = gap.clone() + &" ".repeat(box_size.clone() / 2) + "|";
        let line_7 = gap.clone() + "[ " + &" ".repeat(line_7_padding.clone() / 2) + south + &" ".repeat(line_7_padding.clone() / 2 + line_7_padding.clone() % 2) + " ]";

        write!(f, "\n{}\n{}\n{}\n{}\n{}\n{}\n{}", line_1, line_2, line_3, line_4, line_5, line_6, line_7)
    }
}
