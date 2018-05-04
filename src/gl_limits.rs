use gleam::gl;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct GLLimits {
    pub max_vertex_attribs: u32,
    pub max_tex_size: u32,
    pub max_cube_map_tex_size: u32
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for GLLimits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let values = try!(<[_; 3]>::deserialize(deserializer));
        Ok(GLLimits {
            max_vertex_attribs: values[0],
            max_tex_size: values[1],
            max_cube_map_tex_size: values[2],
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for GLLimits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        [self.max_vertex_attribs, self.max_tex_size, self.max_cube_map_tex_size]
            .serialize(serializer)
    }
}

impl GLLimits {
    pub fn detect(gl_: &gl::Gl) -> GLLimits {
        let mut limit = [0];
        unsafe {
            gl_.get_integer_v(gl::MAX_VERTEX_ATTRIBS, &mut limit);
        }
        let max_vertex_attribs = limit[0] as u32;
        unsafe {
            gl_.get_integer_v(gl::MAX_TEXTURE_SIZE, &mut limit);
        }
        let max_tex_size = limit[0] as u32;
        unsafe {
            gl_.get_integer_v(gl::MAX_CUBE_MAP_TEXTURE_SIZE, &mut limit);
        }
        let max_cube_map_tex_size = limit[0] as u32;
        GLLimits { max_vertex_attribs, max_tex_size, max_cube_map_tex_size }
    }
}
