#![cfg(test)]

use std::convert::Infallible;

use crate as ordinal_map;
use crate::Ordinal;

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct UnitStruct;

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct TupleStruct1(u8);

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct TupleStruct2(u8, bool);

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct RecordStruct1 {
    a: u8,
}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct RecordStruct2 {
    a: u8,
    b: bool,
}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum EmptyEnum {}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum SimpleEnum1 {
    A,
}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum SimpleEnum3 {
    A,
    B,
    C,
}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum TupleEnum {
    A(bool),
    B(Option<()>),
}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum RecordEnum {
    A { a: bool },
    B { a: bool, b: Option<()> },
}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum MixedEnum {
    A,
    B(bool),
    C { a: Result<bool, ()>, b: Option<()> },
}

#[derive(Ordinal, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct EmptyStruct {
    _infallible: Infallible,
}

#[cfg(test)]
mod tests {
    use crate::tests::derive::EmptyEnum;
    use crate::tests::derive::EmptyStruct;
    use crate::tests::derive::MixedEnum;
    use crate::tests::derive::RecordEnum;
    use crate::tests::derive::RecordStruct1;
    use crate::tests::derive::RecordStruct2;
    use crate::tests::derive::SimpleEnum1;
    use crate::tests::derive::SimpleEnum3;
    use crate::tests::derive::TupleEnum;
    use crate::tests::derive::TupleStruct1;
    use crate::tests::derive::TupleStruct2;
    use crate::tests::derive::UnitStruct;
    use crate::tests::util::test_ordinal;

    #[test]
    fn test_unit_struct() {
        test_ordinal([UnitStruct]);
    }

    #[test]
    fn test_tuple_struct_1() {
        test_ordinal((0..=255).map(|i| TupleStruct1(i)));
    }

    #[test]
    fn test_tuple_struct_2() {
        test_ordinal((0..=255).flat_map(|i| [false, true].map(move |b| TupleStruct2(i, b))));
    }

    #[test]
    fn test_record_struct_1() {
        test_ordinal((0..=255).map(|i| RecordStruct1 { a: i }));
    }

    #[test]
    fn test_record_struct_2() {
        test_ordinal((0..=255).flat_map(|i| [false, true].map(move |b| RecordStruct2 { a: i, b })));
    }

    #[test]
    fn test_empty_enum() {
        test_ordinal::<EmptyEnum>([]);
    }

    #[test]
    fn test_simple_enum_1() {
        test_ordinal([SimpleEnum1::A]);
    }

    #[test]
    fn test_simple_enum_3() {
        test_ordinal([SimpleEnum3::A, SimpleEnum3::B, SimpleEnum3::C]);
    }

    #[test]
    fn test_tuple_enum() {
        test_ordinal([
            TupleEnum::A(false),
            TupleEnum::A(true),
            TupleEnum::B(None),
            TupleEnum::B(Some(())),
        ]);
    }

    #[test]
    fn test_record_enum() {
        test_ordinal([
            RecordEnum::A { a: false },
            RecordEnum::A { a: true },
            RecordEnum::B { a: false, b: None },
            RecordEnum::B {
                a: false,
                b: Some(()),
            },
            RecordEnum::B { a: true, b: None },
            RecordEnum::B {
                a: true,
                b: Some(()),
            },
        ]);
    }

    #[test]
    fn test_mixed_enum() {
        test_ordinal([
            MixedEnum::A,
            MixedEnum::B(false),
            MixedEnum::B(true),
            MixedEnum::C {
                a: Ok(false),
                b: None,
            },
            MixedEnum::C {
                a: Ok(false),
                b: Some(()),
            },
            MixedEnum::C {
                a: Ok(true),
                b: None,
            },
            MixedEnum::C {
                a: Ok(true),
                b: Some(()),
            },
            MixedEnum::C {
                a: Err(()),
                b: None,
            },
            MixedEnum::C {
                a: Err(()),
                b: Some(()),
            },
        ]);
    }

    #[test]
    fn test_empty_struct() {
        test_ordinal::<EmptyStruct>([]);
    }
}
