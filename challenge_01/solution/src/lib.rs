pub enum Color {
    RGB {
        red: u8,
        green: u8,
        blue: u8
    },
    HSV {
        hue: u16,
        saturation: u8,
        value: u8,
    }
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::RGB { red, green, blue } => format!("#{:0>2x}{:0>2x}{:0>2x}", red, green, blue),
            Color::HSV { hue, saturation, value } => format!("hsv({},{}%,{}%)", hue.to_string(), saturation.to_string(), value.to_string()),
        }
    }

    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Self {
        let color = Color::RGB {
            red: red,
            green: green,
            blue: blue,
        };

        return color;
    }
    
    pub fn new_hsv(hue: u16, saturation: u8, value: u8) -> Self {
        if hue <=360 && saturation <= 100 && value <= 100 {
            let color = Color::HSV {
                hue: hue,
                saturation: saturation,
                value: value,
            };
    
            return color;
        } else if hue > 360 {
            panic!("Invalid Hue");
        } else if saturation > 100 {
            panic!("Invalid Saturation");
        } else {
            panic!("Invalid Value");
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            Color::RGB { red, green, blue } => Color::RGB { red: u8::MAX - red, green: u8::MAX - green, blue: u8::MAX - blue, },
            Color::HSV { hue, saturation, value } => Color::HSV { hue: 360 - hue, saturation: 100 - saturation, value: 100 - value, },
        }
    }
    
    pub fn unwrap_rgb(&self) -> (u8, u8, u8) {
        if let &Color::RGB { red, green, blue } = self {
            return (red, green, blue);
        } else {
            panic!("Invalid!");
        }
    }

    
    pub fn unwrap_hsv(&self) -> (u16, u8, u8) {
        if let &Color::HSV { hue, saturation, value } = self {
            return (hue, saturation, value);
        } else {
            panic!("Invalid!");
        }
    }
}
