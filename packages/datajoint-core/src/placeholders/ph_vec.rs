use crate::placeholders::ph_arg::PhArg;
use sqlx::query::Query;
use sqlx::Any;
use sqlx::database::HasArguments;
use crate::placeholders::PlaceHolderArgument;
use crate::types::DecodeResult;

pub struct PlaceHolderArgumentVector {
    pub vec : Vec<PlaceHolderArgument>
}


impl PlaceHolderArgumentVector {

    pub fn new( vec : Vec<PlaceHolderArgument>) -> Self{
        PlaceHolderArgumentVector{
            vec: vec![],
        }
    }

    pub fn prepare(self, query : &str) -> Query<Any, <Any as HasArguments>::Arguments> {
        let mut qu = sqlx::query::<Any>(query);
        for arg in self.vec {
            match arg.data() {
                DecodeResult::Int8(a) => {qu = qu.bind(a as i32)}
                DecodeResult::UInt8(a) => {qu = qu.bind(a as i32)}
                DecodeResult::Int16(a) => {qu = qu.bind(a as i32)}
                DecodeResult::UInt16(a) => {qu = qu.bind(a as i32)}
                DecodeResult::Int32(a) => {qu = qu.bind(a)}
                DecodeResult::UInt32(a) => {qu = qu.bind(a as i32)}
                DecodeResult::String(a) => {qu = qu.bind(a)}
                DecodeResult::Float32(a) => {qu = qu.bind(a)}
                DecodeResult::Float64(a) => {qu = qu.bind(a)}
                DecodeResult::Bytes(a) => {qu = qu.bind(a)}

            };
        };
        qu
    }

    pub fn add(&mut self, arg: PlaceHolderArgument) {
        self.vec.push(arg)
    }

    pub fn get(&self, index : usize) -> &PlaceHolderArgument {
        &self.vec[index]
    }

    pub fn size(self) -> usize {
        self.vec.len()
    }

    pub fn set_data(&mut self, index : usize, arg: PlaceHolderArgument) {
        self.vec.insert(index,arg)
    }
}
