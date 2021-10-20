use crate::placeholders::PlaceholderArgument;
use crate::types::DecodeResult;
use sqlx::database::HasArguments;
use sqlx::query::Query;
use sqlx::Any;

pub struct PlaceholderArgumentVector {
    pub vec: Vec<PlaceholderArgument>,
}

impl PlaceholderArgumentVector {
    pub fn new(vec: Vec<PlaceholderArgument>) -> Self {
        PlaceholderArgumentVector { vec: vec }
    }

    pub fn prepare(self, query: &str) -> Query<Any, <Any as HasArguments>::Arguments> {
        let mut qu = sqlx::query::<Any>(query);
        for arg in self.vec {
            match arg.into_data() {
                DecodeResult::Int8(a) => qu = qu.bind(a as i32),
                DecodeResult::UInt8(a) => qu = qu.bind(a as i32),
                DecodeResult::Int16(a) => qu = qu.bind(a as i32),
                DecodeResult::UInt16(a) => qu = qu.bind(a as i32),
                DecodeResult::Int32(a) => qu = qu.bind(a),
                // TODO(EdwardGarmon): Will eventually move to using
                // sqlx type parameters so we can encode types correctly
                // according to database type, for now there
                // will be a possible overflow error here
                DecodeResult::UInt32(a) => qu = qu.bind(a as i32),
                DecodeResult::String(a) => qu = qu.bind(a),
                DecodeResult::Float32(a) => qu = qu.bind(a),
                DecodeResult::Float64(a) => qu = qu.bind(a),
                DecodeResult::Bytes(a) => qu = qu.bind(a),
            };
        }
        qu
    }

    pub fn add_arg(&mut self, arg: PlaceholderArgument) {
        self.vec.push(arg)
    }

    pub fn get(&self, index: usize) -> &PlaceholderArgument {
        &self.vec[index]
    }

    pub fn size(self) -> usize {
        self.vec.len()
    }

    pub fn set_data(&mut self, index: usize, arg: PlaceholderArgument) {
        self.vec.insert(index, arg)
    }
}
