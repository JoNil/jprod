use core::marker::PhantomData;
use super::Context;

pub struct QueryManager {
    marker: PhantomData<*const u32>,
}

impl QueryManager {
    #[inline]
    pub fn new(_: &Context) -> QueryManager {
        QueryManager {
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn query<'a>(&'a self) -> Query<'a> {
        Query {
            marker: PhantomData
        }
    }

    #[inline]
    pub fn submit_zones(&mut self) {
    }
}

pub struct Query<'a> {
    marker: PhantomData<&'a QueryManager>,
}