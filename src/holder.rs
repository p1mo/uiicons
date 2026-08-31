use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use speedy::Readable;




#[derive(Debug, Default, speedy::Readable, speedy::Writable)]
/// ### `EmbededIcons`
/// 
/// Holds the icons
pub struct EmbededIcons(HashMap<String, Vec<u8>>);

unsafe impl Send for EmbededIcons {}
unsafe impl Sync for EmbededIcons {}

impl EmbededIcons {
    
    #[cfg(feature = "build")]
    pub fn new(map: HashMap<String, Vec<u8>>) -> Self {
        Self(map)
    }
    
    /// ### Load Icon Binary used by proc macro
    pub fn load(buffer: &[u8]) -> Self {
        EmbededIcons::read_from_buffer(buffer).unwrap()
    }

    /// ### Get Length
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// ### Get Icon
    pub fn get(&self, name: &str) -> Option<&Vec<u8>> {
        self.0.get(name)
    }

    /// ### Find Icon
    pub fn find(&self, name: &str) -> Option<&Vec<u8>> {
        self.0.iter().find(|(ico_name, _)| *ico_name == name).map(|item| item.1)
    }

    /// ### Find Icon (in parallel)
    pub fn par_find(&self, name: &str) -> Option<&Vec<u8>> {
        self.0.par_iter().find_any(|(ico_name, _)| *ico_name == name).map(|item| item.1)
    }
  
    /// ### Iterate over icons
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Vec<u8>> {
        self.0.iter()
    }

}



impl<'a> IntoIterator for &'a EmbededIcons {

    type Item = (&'a String, &'a Vec<u8>);
    type IntoIter = std::collections::hash_map::Iter<'a, String, Vec<u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }

}