use core::ops;

#[derive(Clone, Copy)]
pub struct ClassicalBit<'a> {
    state: bool,
    name: &'a str,
}

#[derive(Clone)]
pub struct ClassicalRegister<'a> {
    bits: Vec<ClassicalBit<'a>>,
    name: &'a str,
}

impl<'a> ClassicalBit<'a> {
    pub fn new(name: &'a str, state: bool) -> ClassicalBit<'a> {
        ClassicalBit { name, state }
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn get_state(&self) -> bool {
        self.state
    }
}

impl<'a> ClassicalRegister<'a> {
    pub fn new(name: &'a str, names: &'a [&'a str]) -> ClassicalRegister<'a> {
        let mut bits: Vec<ClassicalBit<'a>> = Vec::new();
        for i in 0..names.len() {
            bits.push(ClassicalBit::new(names[i], false));
        }
        ClassicalRegister { name, bits }
    }

    pub fn set_bits(&mut self, bits: Vec<ClassicalBit<'a>>) {
        self.bits = bits;
    }

    pub fn get_bits(&self) -> Vec<ClassicalBit<'a>> {
        self.bits.clone()
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a> ops::Index<usize> for ClassicalRegister<'a> {
    type Output = ClassicalBit<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl<'a> ops::IndexMut<usize> for ClassicalRegister<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bits[index]
    }
}
