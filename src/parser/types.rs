pub type BytesRef = std::sync::Arc<Vec<u8>>;

#[derive(Clone, PartialEq)]
pub enum Type {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    //the 1st, container class
    //the 2nd, generic class's arg
    //the 3rd, if there is a '+'
    //  Ljava/util/List<Lcom/google/inject/Module;>;)
    //    => java.util.List<com.google.inject.Module>
    //  Ljava/lang/Class<+Lcom/google/inject/Module;>;
    //    => java.lang.Class<? extends com.google.inject.Module>
    Object(BytesRef, Option<Vec<Type>>, Option<u8>),
    Short,
    Boolean,
    Array(BytesRef),
    Void,
}
