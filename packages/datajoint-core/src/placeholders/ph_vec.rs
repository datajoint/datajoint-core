use crate::placeholders::PlaceholderArgument;
use crate::types::NativeType;
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
                NativeType::None => todo!(),
                NativeType::Int8(a) => qu = qu.bind(a as i32),
                NativeType::UInt8(a) => qu = qu.bind(a as i32),
                NativeType::Int16(a) => qu = qu.bind(a as i32),
                NativeType::UInt16(a) => qu = qu.bind(a as i32),
                NativeType::Int32(a) => qu = qu.bind(a),
                // TODO(EdwardGarmon): Will eventually move to using
                // sqlx type parameters so we can encode types correctly
                // according to database type, for now there
                // will be a possible overflow error here
                NativeType::UInt32(a) => qu = qu.bind(a as i32),
                NativeType::String(a) => qu = qu.bind(a),
                NativeType::Float32(a) => qu = qu.bind(a),
                NativeType::Float64(a) => qu = qu.bind(a),
                NativeType::Bytes(a) => qu = qu.bind(a),
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
