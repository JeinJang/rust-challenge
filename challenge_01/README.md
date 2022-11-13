# CSS Colors

Some students say to themselves, "I'm not going to study math, I'm going to write web". They then spent several hours trying to center the divs and wished they had studied the differential equations and walked to CERN for example.

Well, today is not the time all the way so hard ... but we can still write a little math related to the web, even if it comes down to collecting and subtracting numbers.

Let's implement some methods of the following type:

```rust
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
```

This is an enumbered type that has two options:
- `RGB` with fields for red, green and blue
- `HSV` for hue, saturation and value

The first is probably relatively clear - the values of the three color channels go from 0 to 255 and the mixing of colors gives us the whole spectrum. Zero on all channels gives us black, and 255 will be pure white. If we put 255 in red and green, we get bright yellow.

The second option is a little more interesting - the first value is degree and goes from 0 to 360. This is a value that determines the color relative to its position of some specific color wheel. The values "saturation" and "value" determine the brightness and brightness of this color, from 0% to 100%. Or something like that, the whole thing is terribly unintuitive: https://en.wikipedia.org/wiki/HSL_and_HSV. This animation explains almost nothing, but it's pretty cool: https://twitter.com/DanHollick/status/1578071950123057152

For this challenge, we will not do something too complicated with colors. Let's make convenient constructors first:

```rust
impl Color {
    /// Constructs a new value from variant `RGB` with the given red, green, and blue values.
    ///
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        todo!()
    }

    /// Constructs a new value from variant `HSV` with the given values.
    ///
    /// In case hue is above 360 ​​or saturation or value is above 100, we expect you to `panic!` with
    /// whatever message you choose.
    ///
    pub fn new_hsv(hue: u16, saturation: u8, value: u8) -> Color {
        todo!()
    }
}
```

Pay attention to the panic note. We will usually deal with invalid input in other ways, but we haven't gotten there yet, so panic! in case of invalid entrance we will do the job.

By the way, there is not much point in validating RGB input - if you pass a number above 255, this would be a compilation error. Nice.

To make it easier for us to "take out the pieces" of both options, let's implement methods that just give them to us, as long as we are super-duper sure we are asking for the right option:

```rust
impl Color {
    /// If `self` is `RGB`, then you return its `red`, `green`, `blue` components in that order.
    /// Otherwise, `panic!` with whatever message you choose.
    ///
    pub fn unwrap_rgb(&self) -> (u8, u8, u8) {
        todo!()
    }

    /// If `self` is `HSV` then you return its `hue`, `saturation`, `value` components in this
    /// row. Otherwise, `panic!` with whatever message you choose.
    ///
    pub fn unwrap_hsv(&self) -> (u16, u8, u8) {
        todo!()
    }
}
```

Notice that you are receiving self by reference. That makes sense - just read that value. But you will have to return the numbers as values. This is also logical, you are rarely particularly useful pointers to numbers.

If we are going to use these structures for CSS, we need to format them as for CSS. Let's print the color as something the CSS will understand:

```rust
impl Color {
    /// In case the variant of `self` is `RGB`, we expect a string with the content `#rrggbb`, where
    /// the red, green, and blue components are formatted in hexadecimal, and each is
    /// exactly two lowercase (zero-padded) characters.
    ///
    /// If the variant is `HSV`, we expect a string `hsv(h,s%,v%)` where the numbers are typed in
    /// the decimal system, no leading zeros, no spaces after the commas, the second two ending in
    /// `%`.
    ///
    pub fn to_string(&self) -> String {
        todo!()
    }
}
```

For example:

```rust
println!("{}", Color::new_rgb(0, 0, 0).to_string());     //=> #000000
println!("{}", Color::new_rgb(255, 1, 255).to_string()); //=> #ff01ff

println!("{}", Color::new_hsv(0, 0, 0).to_string());       //=> hsv(0,0%,0%)
println!("{}", Color::new_hsv(360, 100, 100).to_string()); //=> hsv(360,100%,100%)
```

If you think how annoying it would be to format the numbers in hexadecimal form with padding from scratch ... maybe somewhere in the slides of the lectures you will find some hint of how to make it quite easy. Maybe you will find exactly these two things used in a lecture.

The last thing we want is to write a little logic for inverting two colors. In the case of RGB, the opposite of `#00ff00`` it will be #ff00ff`, in the case of HSV, invert `hsv(0,50%,0%)` will be `hsv(360,50%,100%)`. That is, for each individual component we take the addition to the maximum. We're not sure if this makes sense to HSV, but we're programmers, we're not ... "color scientists"?

```rust
impl Color {
    /// Inverts color componentwise -- for each of the values, the difference with the maximum is taken.
    ///
    pub fn invert(&self) -> Self {
        todo!()
    }
}
```

Make sure your decision is compiled with the base test:

```rust
use solution::*;

#[test]
fn test_basic() {
    let color1 = Color::new_rgb(0, 0, 0);
    assert_eq!(color1.unwrap_rgb().0, 0);
    assert_eq!(&color1.to_string()[0..1], "#");

    let color2 = Color::new_hsv(0, 0, 0);
    assert_eq!(color2.unwrap_hsv().0, 0);

    assert_eq!(color1.invert().unwrap_rgb().0, 255);
}
```