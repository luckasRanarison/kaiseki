pub const TERM_FST: &[u8] = include_bytes!("../../../data/term.fst");
pub const UNK_DICT: &[u8] = include_bytes!("../../../data/unk.bin");
pub const TERM_DICT: &[u8] = include_bytes!("../../../data/dict.bin");
pub const CHAR_TABLE: &[u8] = include_bytes!("../../../data/char.bin");
pub const COST_MATRIX: &[u8] = include_bytes!("../../../data/matrix.bin");

#[cfg(test)]
mod tests {
    use super::*;

    use mecab_types::{
        bincode::decode_slice,
        char::{CharCategory, CharTable},
        cost::CostMatrix,
        unk::UnknownDictionary,
    };

    #[test]
    fn test_get_term() {
        let unk_dict = decode_slice::<UnknownDictionary>(UNK_DICT).unwrap();
        let terms = unk_dict.get_terms("DEFAULT").unwrap();
        let (id, term) = &terms[0];

        assert_eq!(0, *id);
        assert_eq!(5, term.context_id);
        assert_eq!(4769, term.cost);
    }

    #[test]
    fn test_lookup() {
        let lookup_table = decode_slice::<CharTable>(CHAR_TABLE).unwrap();
        let categories = lookup_table.lookup('一');
        let expected = vec![
            CharCategory::new("KANJINUMERIC".to_owned(), true, true, 0),
            CharCategory::new("KANJI".to_owned(), false, false, 2),
        ];

        assert_eq!(&expected, categories);

        let categories = lookup_table.lookup('1');
        let expected = vec![CharCategory::new("NUMERIC".to_owned(), true, true, 0)];

        assert_eq!(&expected, categories);
    }

    #[test]
    fn test_cost_matrix() {
        let cost_matrix = decode_slice::<CostMatrix>(COST_MATRIX).unwrap();
        let value = cost_matrix.get(0, 0);

        assert_eq!(value, -434);
    }
}
