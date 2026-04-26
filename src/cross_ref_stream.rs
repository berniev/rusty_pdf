use crate::xref_ops::{Generation, ObjectStatus};

/// Cross-Reference Streams
/// Beginning with PDF 1.5, cross-reference information may be stored in a cross-reference stream
/// instead of in a cross-reference table. Cross-reference streams provide the following advantages:
/// • More compact representation of cross-reference information
/// • Ability to access compressed objects that are stored in object streams (see 7.5.7,
///   "Object Streams") and to allow new cross-reference entry types to be added in the future.
///
/// Cross-reference streams are stream objects.
/// Cross-reference streams contain a dictionary and a data stream.
///
/// Each cross-reference stream contains the information equivalent
/// to the cross-reference table (see 7.5.4, "Cross-Reference Table") and trailer (see 7.5.5, "File
/// Trailer") for one cross-reference section.
/// 
///  Entry types:
///  =========================================================================
///  Type                  Num  Field2         Field3
///  ====================  ===  =============  ===============================
///  Free object           0    Next free obj  Generation (65535 for object 0)
///  Uncompressed          1    Byte offset    Generation
///  Compressed in objstm  2    objstm number  Index within objstm
///  =========================================================================
///

//--------------------------- CrossRefEntry -------------------------//

#[derive(Clone)]
pub enum CrossRefStreamEntry {
    FreeObject {
        next_free_obj: usize,
        generation: u16,
    },
    Uncompressed {
        byte_offset: usize,
        generation: u16,
    },
    CompressedInObjstm {
        objstm_number: usize,
        index_within_objstm: u16,
    },
}

impl CrossRefStreamEntry {
    pub fn from_object_metadata(
        status: ObjectStatus,
        offset: usize,
        generation: u16,
        compression_info: Option<(usize, usize)>,
    ) -> Self {
        if status == ObjectStatus::Free {
            let object_number = offset; // shut up
            CrossRefStreamEntry::FreeObject {
                next_free_obj: 0,
                generation: if object_number == 0 {
                    Generation::ROOT_GENERATION
                } else {
                    0
                },
            }
        } else if let Some((objstm_num, index)) = compression_info {
            CrossRefStreamEntry::CompressedInObjstm {
                objstm_number: objstm_num,
                index_within_objstm: index as u16,
            }
        } else {
            CrossRefStreamEntry::Uncompressed {
                byte_offset: offset,
                generation,
            }
        }
    }

    /// Calculate minimum bytes needed to represent a usize value
    fn bytes_needed_usize(value: usize) -> usize {
        if value == 0 {
            return 1;
        }
        // Calculate ceil(log256(val)) = number of bytes needed
        let bits = usize::BITS - value.leading_zeros();
        ((bits + 7) / 8) as usize
    }

    /// Calculate minimum bytes needed to represent a u16 value
    fn bytes_needed_u16(value: u16) -> usize {
        if value == 0 {
            return 1;
        }
        // Calculate ceil(log256(val)) = number of bytes needed
        let bits = 16 - value.leading_zeros();
        ((bits + 7) / 8) as usize
    }

    /// Returns the type byte for this cross-reference entry according to PDF spec
    pub fn type_byte(&self) -> u8 {
        match self {
            CrossRefStreamEntry::FreeObject { .. } => 0,
            CrossRefStreamEntry::Uncompressed { .. } => 1,
            CrossRefStreamEntry::CompressedInObjstm { .. } => 2,
        }
    }

    /// Returns minimum (field2_width, field3_width) needed for this entry
    pub fn required_widths(&self) -> (usize, usize) {
        match self {
            CrossRefStreamEntry::FreeObject {
                next_free_obj,
                generation,
            } => (
                Self::bytes_needed_usize(*next_free_obj),
                Self::bytes_needed_u16(*generation),
            ),
            CrossRefStreamEntry::Uncompressed {
                byte_offset,
                generation,
            } => (
                Self::bytes_needed_usize(*byte_offset),
                Self::bytes_needed_u16(*generation),
            ),
            CrossRefStreamEntry::CompressedInObjstm {
                objstm_number,
                index_within_objstm,
            } => (
                Self::bytes_needed_usize(*objstm_number),
                Self::bytes_needed_u16(*index_within_objstm),
            ),
        }
    }
}

//--------------------------- CrossRefStream -------------------------//

//#[allow(dead_code)]
pub(crate) struct CrossRefStream {
    entries: Vec<CrossRefStreamEntry>,
}
impl CrossRefStream {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut stream = CrossRefStream {
            entries: Vec::new(),
        };
        stream.entries.push(CrossRefStreamEntry::FreeObject {
            next_free_obj: 0,
            generation: Generation::ROOT_GENERATION,
        });
        stream
    }

    #[allow(dead_code)]
    pub fn add_entry(&mut self, entry: CrossRefStreamEntry) {
        self.entries.push(entry);
    }

    /// Calculate optimal field widths based on actual entry data
    /// Returns (field2_width, field3_width)
    #[allow(dead_code)]
    pub fn calculate_optimal_widths(&self) -> (usize, usize) {
        self.entries
            .iter()
            .map(|entry| entry.required_widths())
            .fold((1, 1), |(max2, max3), (w2, w3)| {
                (max2.max(w2), max3.max(w3))
            })
    }

    #[allow(dead_code)]
    pub fn build_binary_data(&self, field2_width: usize, field3_width: usize) -> Vec<u8> {
        let mut data = Vec::new();
        for entry in &self.entries {
            data.push(entry.type_byte());

            let (field2, field3) = match entry {
                CrossRefStreamEntry::FreeObject {
                    next_free_obj,
                    generation,
                } => (*next_free_obj, *generation),
                CrossRefStreamEntry::Uncompressed {
                    byte_offset,
                    generation,
                } => (*byte_offset, *generation),
                CrossRefStreamEntry::CompressedInObjstm {
                    objstm_number,
                    index_within_objstm,
                } => (*objstm_number, *index_within_objstm),
            };

            // Encode field2 in big-endian
            let field2_bytes = field2.to_be_bytes();
            data.extend_from_slice(&field2_bytes[8 - field2_width..]);

            // Encode field3 in big-endian
            let field3_bytes = field3.to_be_bytes();
            data.extend_from_slice(&field3_bytes[2 - field3_width..]);
        }
        data
    }

    #[allow(dead_code)]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Build the complete xref stream object with dictionary and binary data
    /// Returns (stream_bytes, xref_data_length) for writing to PDF
    #[allow(dead_code)]
    pub fn build_stream_object(
        &self,
        stream_num: usize,
        root_obj_id: usize,
        info_obj_id: Option<usize>,
    ) -> Vec<u8> {
        let (field2_width, field3_width) = self.calculate_optimal_widths();
        let xref_data = self.build_binary_data(field2_width, field3_width);

        let w_array_str = format!("[ 1 {} {} ]", field2_width, field3_width);
        let total_entries = self.entry_count();

        let mut dict_entries = vec![
            "/Type /XRef".to_string(),
            format!("/Size {}", total_entries),
            format!("/Root {} 0 R", root_obj_id),
            format!("/W {}", w_array_str),
        ];

        if let Some(info_id) = info_obj_id {
            dict_entries.push(format!("/Info {} 0 R", info_id));
        }

        dict_entries.push(format!("/Length {}", xref_data.len()));

        let dict_str = format!("<< {} >>", dict_entries.join(" "));
        let mut stream_bytes = Vec::new();
        stream_bytes
            .extend_from_slice(format!("{} 0 obj\n{}\nstream\n", stream_num, dict_str).as_bytes());
        stream_bytes.extend_from_slice(&xref_data);
        stream_bytes.extend_from_slice(b"\nendstream\nendobj");

        stream_bytes
    }
}
