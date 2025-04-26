#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8, // Red channel
    pub g: u8, // Green channel
    pub b: u8, // Blue channel
    pub a: u8, // Alpha (transparency) channel
}

impl Color {
    // Swaps the values of two color components based on their current values
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match first {
            // If 'first' matches the red channel
            v if v == self.r => match second {
                // Swap red with blue
                v if v == self.b => {
                    self.r = second;
                    self.b = first;
                }
                // Swap red with green
                v if v == self.g => {
                    self.r = second;
                    self.g = first;
                }
                // Swap red with alpha
                v if v == self.a => {
                    self.r = second;
                    self.a = first;
                }
                // Do nothing if no match
                _ => {}
            },

            // If 'first' matches the green channel
            v if v == self.g => match second {
                // Swap green with red
                v if v == self.r => {
                    self.g = second;
                    self.r = first;
                }
                // Swap green with blue
                v if v == self.b => {
                    self.g = second;
                    self.b = first;
                }
                // Swap green with alpha
                v if v == self.a => {
                    self.g = second;
                    self.a = first;
                }
                // Do nothing if no match
                _ => {}
            },

            // If 'first' matches the alpha channel
            v if v == self.a => match second {
                // Swap alpha with red
                v if v == self.r => {
                    self.a = second;
                    self.r = first;
                }
                // Swap alpha with blue
                v if v == self.b => {
                    self.a = second;
                    self.b = first;
                }
                // Swap alpha with green
                v if v == self.g => {
                    self.a = second;
                    self.g = first;
                }
                // Do nothing if no match
                _ => {}
            },

            // If 'first' matches the blue channel
            v if v == self.b => match second {
                // Swap blue with red
                v if v == self.r => {
                    self.b = second;
                    self.r = first;
                }
                // Swap blue with green
                v if v == self.g => {
                    self.b = second;
                    self.g = first;
                }
                // Swap blue with alpha
                v if v == self.a => {
                    self.b = second;
                    self.a = first;
                }
                // Do nothing if no match
                _ => {}
            },

            // Do nothing if 'first' doesn't match any channel
            _ => {}
        }

        // Return the updated color
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };
        // swap r
        assert_eq!(
            c.swap(c.r, c.b),
            Color {
                r: 10,
                g: 200,
                b: 255,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.r, c.g),
            Color {
                r: 200,
                g: 255,
                b: 10,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.r, c.a),
            Color {
                r: 30,
                g: 200,
                b: 10,
                a: 255
            }
        );

        // swap g
        assert_eq!(
            c.swap(c.g, c.r),
            Color {
                r: 200,
                g: 255,
                b: 10,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.g, c.b),
            Color {
                r: 255,
                g: 10,
                b: 200,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.g, c.a),
            Color {
                r: 255,
                g: 30,
                b: 10,
                a: 200
            }
        );

        // swap b
        assert_eq!(
            c.swap(c.b, c.r),
            Color {
                r: 10,
                g: 200,
                b: 255,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.b, c.g),
            Color {
                r: 255,
                g: 10,
                b: 200,
                a: 30
            }
        );
        assert_eq!(
            c.swap(c.b, c.a),
            Color {
                r: 255,
                g: 200,
                b: 30,
                a: 10
            }
        );

        // swap a
        assert_eq!(
            c.swap(c.a, c.r),
            Color {
                r: 30,
                g: 200,
                b: 10,
                a: 255
            }
        );
        assert_eq!(
            c.swap(c.a, c.b),
            Color {
                r: 255,
                g: 200,
                b: 30,
                a: 10
            }
        );
        assert_eq!(
            c.swap(c.a, c.g),
            Color {
                r: 255,
                g: 30,
                b: 10,
                a: 200
            }
        );
    }
}
